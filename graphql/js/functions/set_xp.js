import { getLevel, levelNeededXP } from "./format.js"

export async function setXP(data = handledData){
    setGoXP(data)
    setJSXP(data)
    setDivXP(data)

}

async function setJSXP(data){
    let xp = await getXP(data, 'piscine-js')
    let level = getLevel(xp)
    let requiredXP = levelNeededXP(level+1) -  levelNeededXP(level)
    let lvlLbl = document.getElementById('js-lvl')
    let xpLbl = document.getElementById('js-xp')
    let xpLeftOver = xp -  levelNeededXP(level)
    let difference = (xpLeftOver / requiredXP).toFixed(2) * 100

    lvlLbl.innerText = "level: " + level
    xpLbl.style.width = difference + "%"
    xpLbl.innerHTML = xpLeftOver
}

async function setDivXP(data){
    let xp = await getXP(data, 'div-01')
    let level = getLevel(xp)
    let requiredXP = levelNeededXP(level+1) -  levelNeededXP(level)
    let lvlLbl = document.getElementById('div-lvl')
    let xpLbl = document.getElementById('div-xp')
    let xpLeftOver = xp -  levelNeededXP(level)
    let difference = (xpLeftOver / requiredXP).toFixed(2) * 100

    lvlLbl.innerText = "level: " + level
    xpLbl.style.width = difference + "%"
    xpLbl.innerHTML = xpLeftOver
}

async function setGoXP(data){
    let xp = await getXP(data, 'piscine-go')

    let level = getLevel(xp)
    let requiredXP = levelNeededXP(level+1) -  levelNeededXP(level)
    let lvlLbl = document.getElementById('go-lvl')
    let xpLbl = document.getElementById('go-xp')
    let xpLeftOver = xp -  levelNeededXP(level)
    let difference = (xpLeftOver / requiredXP).toFixed(2) * 100

    lvlLbl.innerText = "level: " + level
    xpLbl.style.width = difference + "%"
    xpLbl.innerHTML = xpLeftOver
}

async function getXP(dataArray, type){
    let xp = 0 
    dataArray.forEach(data => {
        if (data.from == type){
            xp += data.xp
        }
    });

    return xp
}

async function getTopXP(){
    let topQuery = `{
        user(limit: 1, order_by: {transactions_aggregate: {sum: {amount: desc_nulls_last}}}, where: {transactions: {type: {_eq: "xp"}}}) {
            id
            login
        }
    }`


    let topID = (await fetchDataWithQuery(topQuery)).data.user[0].id

    return topID
}