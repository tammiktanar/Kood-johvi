import { getDateFormat, getXPFormatStr } from "./format.js"

export async function setProjects(data){
    setLatestProjects(data)
    setPiscineGoProjects(data)
    setDIV01Projects(data)
    setPiscineJSProjects(data)
    setAllProjects(data)
}

async function setLatestProjects(data = handledData){
    let table = document.getElementById('latests-projects')
    $(`#${table.id}`).find("tr:gt(0)").remove();
    let nr = 0

    if (data.length < 8) { nr = data.length} else { nr = 8 }

    for (let i = 0; i < nr ; i++) {
        const element = data[i];

        tableAddRow(element, table, i)
        
    }
}

async function setPiscineGoProjects(data = handledData){
    let table = document.getElementById('piscine-go-projects')
    $(`#${table.id}`).find("tr:gt(0)").remove();
    let nr = data.length
    let k = 0;
    for (let i = 0; i < nr ; i++) {
        const element = data[i];

        if (element.from == "piscine-go") {
            tableAddRow(element, table, k)
            k++
        } 
    }
}

async function setDIV01Projects(data = handledData){
    let table = document.getElementById('div-01-projects')
    $(`#${table.id}`).find("tr:gt(0)").remove();
    let nr = data.length
    let k = 0;
    for (let i = 0; i < nr ; i++) {
        const element = data[i];

        if (element.from == "div-01") {
            tableAddRow(element, table, k)
            k++
        } 
    }
}

async function setPiscineJSProjects(data = handledData){
    let table = document.getElementById('piscine-js-projects')
    $(`#${table.id}`).find("tr:gt(0)").remove();
    let nr = data.length
    let k = 0;
    for (let i = 0; i < nr ; i++) {
        const element = data[i];

        if (element.from == "piscine-js") {
            tableAddRow(element, table, k)
            k++
        } 
    }
}

async function setAllProjects(data = handledData) {
    let table = document.getElementById('all-projects')
    $(`#${table.id}`).find("tr:gt(0)").remove();
    let nr = data.length
    for (let i = 0; i < nr ; i++) {
        const element = data[i];
        tableAddRow(element, table, i)
    }
}

function tableAddRow(data, table, i){
    let row = table.insertRow(i+1)
    let cell1 = row.insertCell(0)
    let cell2 = row.insertCell(1)
    let cell3 = row.insertCell(2)
    let cell4 = row.insertCell(3)

    
    cell1.innerHTML = i+1
    setTooltip(cell1, data.objectId)

    cell2.innerHTML = data.name
    setTooltip(cell2, data.path)
    
    cell3.innerHTML = getXPFormatStr(data.xp)
    setTooltip(cell3, data.xp)

    cell4.innerHTML = getDateFormat(data.createdAt)
    setTooltip(cell4, data.createdAt)
}

function setTooltip(element, tip, where = "top"){
    element.title = tip
    element.setAttribute("data-bs-toggle", 'tooltip')
    element.setAttribute("data-bs-placement", where)
}

export function tableSetLoading(table){
    $(`#${table.id}`).find("tr:gt(0)").remove();
    let row = table.insertRow(1)
    let cell1 = row.insertCell(0)

    cell1.colSpan = 4
    cell1.innerText = "Loading..."
}


