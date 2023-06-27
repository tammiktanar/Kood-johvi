import * as queries from "./graphql.js"





// Show an error to the user
function showError(errorText) {

}

// Toggle between showing the content and a loading icon
function loading(bool) {

}





const sprintToRegex = {
	"div-01": "^/johvi/div-01/(?!piscine-js(?:-\\d+)?/)",
	"piscine-go": "^/johvi/piscine-go/",
	"piscine-js": "^/johvi/div-01/piscine-js(?:-\\d+)?/",
}

// Load all sections related to the requested user
async function loadUser(username, sprint = "div-01") {
	const user = await findUser(username)

	const projects = await getUserProjects(user.id, sprintToRegex[sprint])
	const xp = projects.reduce((acc, project) => acc + project.xp, 0)


	fillUserInfo(user)
	fillUserLevel(xp)
	fillProjectsTable(projects)

	await Promise.all([fillUserXp(xp, sprint), fillUserAudit(user.id)])
	await fillGraph(projects)

	// console.log(user)
	// console.log(projects.filter(p => p.xp > 0).reverse())
}

// ------------------------- XP GRAPH -------------------------

async function fillGraph(projects) {



	const years = projects[projects.length - 1].date.getFullYear() - projects[0].date.getFullYear()
	const months = projects[projects.length-1].date.getMonth() - projects[0].date.getMonth()
	const divisors = years * 12 + months

	let acc = 0
	const data = {
		series: [{
			name: "",
			data: projects.map(p => {acc += p.xp; return {x: p.date, y: acc};})
		}]
	}

	const options = {
		axisY: {
			labelInterpolationFnc: function(value) {
				return xpToString(value);
			}
		},
		axisX: {
			type: Chartist.FixedScaleAxis,
			divisor: divisors,
			labelInterpolationFnc: function(value) {
				return moment(value).format('MMM/YYYY');
			}
		}
	}

	Chartist.Line('.ct-chart', data, options)
}

// ------------------------- PROJECTS TABLE -------------------------

/**
 *
 * @param {Project[]} projects
 */
function fillProjectsTable(projects) {

}

/** @param {Project} project */
const projectRow = (project) => `
<tr>
	<td>${project.path.substr(6).replace(/\//g, " / ")}</td>
	<td>${xpToString(project.xp)}</td>
	<td>${project.date.toLocaleString('en-GB')}</td>
</tr>
`


// ------------------------- AUDIT RATIO -------------------------

async function fillUserAudit(userID) {
	const up = (await queryAll(queries.getTransactions, (v) => v["transaction"], {user: userID, type: "up"}))
		.reduce((acc, transaction) => acc + transaction["amount"], 0)
	const down = (await queryAll(queries.getTransactions, (v) => v["transaction"], {user: userID, type: "down"}))
		.reduce((acc, transaction) => acc + transaction["amount"], 0)

	const upRatio = up / down
	const downRatio = down / up

}

fillUserAudit(3886)


function ratioToEmoticon(ratio) {
	return isNaN(ratio) ? "" : ratio < 0.5 ? ":(" : ratio < 1 ? ":L" : ratio < 1.5 ? ":)" : ":D"
}



// ------------------------- XP BAR -------------------------

let highestXp = {}

async function getHighestXp(sprint) {
	if (sprint in highestXp) return highestXp[sprint]

	// Not cached

	const user = (await queryAPI(queries.userWithHighestXP)).user[0]
	const projects = await getUserProjects(user.id, sprintToRegex[sprint])
	const xp = projects.reduce((acc, project) => acc + project.xp, 0)

	highestXp[sprint] = xp

	return xp
}

async function fillUserXp(xp, sprint) {
	const highXp = Math.max(xp, await getHighestXp(sprint))

	const percent = xp / highXp * 100

}

function xpToString(xp) {
	if (xp < 1000) {
		return xp + " B"
	} else if (xp < 1000000) {
		return (xp/1000).toPrecision(3) + " kB"
	} else {
		return (xp/1000000).toPrecision(3) + " MB"
	}
}

// ------------------------- LEVEL SECTION -------------------------

function fillUserLevel(xp) {
	const level = calculateLevel(xp)

	const xpProgress = xp - levelNeededXP(level)
	const xpNeeded = levelNeededXP(level+1) - levelNeededXP(level)
	const xpPercent = xpProgress / xpNeeded

}


// ------------------------- BASIC INFO -------------------------

function fillUserInfo(user) {


	/** @type {string[][]} */
	const infos = [
		["Username:", user.name],
		["User ID:", user.id.toString()],
		["First Activity:", user.activeFirst?.toLocaleDateString('en-GB') || "N/A"],
		["Last Activity:", user.activeLast?.toLocaleDateString('en-GB') || "N/A"],
	]

}


// ------------------------- API STUFF -------------------------
/**
 * @typedef User
 * @property {number} id
 * @property {string} name
 * @property {Date} activeFirst
 * @property {Date} activeLast
 */

/**
 * @param username
 * @return {Promise<User>}
 */
async function findUser(username) {
	const res = await queryAPI(queries.userIDByName, {loginExact: username, loginFuzzy: `%${escapeLike(username)}%`})

	/** @type {User} */
	const user = res["exact"]?.[0] || res["fuzzy"]?.[0]
	if (user === undefined) throw new Error(`couldn't find the user "${username}"`)

	const res2 = await queryAPI(queries.firstLastActivity, {user: user.id})

	const first = res2["activity"]["first"]?.[0]?.["date"]
	user.activeFirst = first ? new Date(first) : undefined

	const last = res2["activity"]["last"]?.[0]?.["date"]
	user.activeLast = last ? new Date(last) : undefined

	return user
}

function escapeLike(str) {
	return str.replace(/([%_])/g, "\\$1")
}

/**
 * @typedef Project
 * @property {String} path
 * @property {Number} xp
 * @property {Date} date
 */

/**
 * Gets all project of a given user
 * @param {Number} userID
 * @param {String} regex
 * @returns {promise<Project[]>}
 */
async function getUserProjects(userID, regex = "") {
	/** @type {Project[]} */
	let data = await queryAll(queries.userProjects, data => data["progress"], {user: userID, regex})

	let projects = data.map((o) => ({path: o["path"], date: new Date(o.date)}))

	let promises = projects.map(project => queryAPI(queries.projectXp, {user: userID, path: project.path}))

	let amounts = await Promise.all(promises)

	amounts.forEach((amt, index) => {
		if (amt.transaction.length === 0) {
			// If no transaction
			projects[index].xp = 0
			return
		}

		// If matching transaction
		projects[index].xp = amt.transaction[0].amount
		projects[index].date = new Date(amt.transaction[0].date)
	})

	projects.sort((a, b) => a.date - b.date)

	return projects
}

/**
 * Works like the queryAPI function, but keeps querying with increasing offset until no more data is available.
 *
 * The query must accept a variable $offset
 * The callback must take in response data and return the array that's being offset
 * @param {String} query
 * @param {function(data: Object): Array} callback
 * @param {Object} variables
 * @returns {promise<*[]>}
 */
async function queryAll(query, callback, variables = {}) {
	let result = []
	let offset = 0
	let len = 0

	while (true) {
		const data = await queryAPI(query, {...variables, offset})

		// Extract array from returned data
		const arr = callback(data)
		len = arr.length

		// If no new data, return the result
		if (len === 0) return result

		// Append the array to our result array
		result = [...result, ...arr]
		offset += len
	}
}


/**
 * Sends a GraphQL query and returns the response data in a promise
 * @param {String} query
 * @param {Object} variables
 * @returns {Promise<Object>}
 */
async function queryAPI(query, variables = {}) {
	return fetch("https://01.kood.tech/api/graphql-engine/v1/graphql", {
		method: "POST",
		headers: {"Content-Type": "application/json"},
		body: JSON.stringify({query, variables})
	})
		.then(res => res.json())
		.then(data => {
			if ("errors" in data) throw data.errors
			return data["data"]
		})
}

// Calculates what level this amount of XP would be at
function calculateLevel(xp) {
	let level = 0

	while (levelNeededXP(++level) < xp) {}

	return level-1
}

// Returns the amount of XP needed for any given level
function levelNeededXP(level) {
	return Math.round(level * (176 + 3 * level * (47 + 11 * level)))
}