import { loadProjects } from "./functions/data_managment.js";
import { setGraphs } from "./functions/graph.js";
import { fetchDataWithQuery } from "./functions/query.js";
import { setAudit } from "./functions/set_audit.js";
import { setProjects, tableSetLoading } from "./functions/set_projects.js";
import { setSkills } from "./functions/set_skills.js";
import { setXP } from "./functions/set_xp.js";
import { createUserSearch } from "./functions/user_search.js";

document.onreadystatechange = function () {
    var state = document.readyState;
    if (state == 'complete') {
        createUserSearch()
        loadProfile() 
    }
};








    
    

let handledData = []

export async function loadProfile(id = 3886){
    if (id == null || id == undefined) return
    setLoadingProfile();

    setAudit(id)
    setSkills(id)

    addProfilePicFromID(id)


    handledData = []

    handledData = await loadProjects(id)


    setProjects(handledData)

    setGraphs(handledData)

    setXP(handledData)



}


async function addProfilePicFromID(id = 3886){
    let img = document.getElementById('profile-pic')
    let nameField = document.getElementById('profile-name')
    let idField = document.getElementById('profile-id')

    let getUsernameQuery = `{
        user (where: { id: { _eq: ${id} }}) {
            id
            username: login
          }
    }`

    let data = (await fetchDataWithQuery(getUsernameQuery)).data

    console.log()
    img.src = `https://git.01.kood.tech/user/avatar/${data.user[0].username}/-1`
    nameField.innerText = data.user[0].username
    idField.innerText = "ID: " + data.user[0].id
}

function setLoadingProfile(){
    let tables = [
        'latests-projects',
        'all-projects',
        'piscine-js-projects',
        'div-01-projects',
        'piscine-go-projects',
    ]

    tables.forEach(tableName => {
        let table = document.getElementById(tableName)
        tableSetLoading(table)
    });

    let profilePic = document.getElementById('profile-pic')
    profilePic.src="https://www.kaindl.com/fileadmin/_processed_/d/8/csm_2162_PE_Dekorbild_0ec3e17e00.jpg"

    let canvases = [
        'audit-graph',
        'go-graph',
        'div-graph',
        'js-graph',
        'skills-graph'
    ]

    canvases.forEach(canvasName => {
        const chart = Chart.getChart(canvasName);
        if (chart) chart.destroy()
    });

    let progressBars = [
        'audit-down-rate',
        'audit-up-rate',
        'go-xp',
        'div-xp',
        'js-xp'
    ]

    progressBars.forEach(progressBar => {
        let bar = document.getElementById(progressBar)
        bar.style.width = "0%"
        bar.innerText = ""
    })

    let numbersDivs = [
        'audit-ratio',
        'go-lvl',
        'div-lvl',
        'js-lvl'
    ]

    numbersDivs.forEach(numberDiv => {
        let numberDivElement = document.getElementById(numberDiv)
        numberDivElement.innerHTML = ""
    });


}




















// WEEEEE
