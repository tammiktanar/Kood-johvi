import { fetchDataWithQuery } from "./query.js"

export async function loadProjects(id = 3886){
    let allDoneProjects = await getAllDoneProjects(id)

    let data = await transformProjectData(allDoneProjects, id)

    return data
}


async function getAllDoneProjects(id = 3886, offset = 0, projects = []){
    let allDoneProjectsQuery = `{
        progress (
            where: {
            userId: {_eq: ${id}},
            isDone: {_eq: true}
            },
            offset: ${offset},
            limit: 20,
            distinct_on: path
        ) {
            grade
            path
            createdAt
            objectId
            object {
                type
            }
        }
    }`



    let returned_projects = await (await fetchDataWithQuery(allDoneProjectsQuery))

    if (!returned_projects.errors){
        if (returned_projects.data.progress.length > 0) {
            projects = projects.concat(returned_projects.data.progress)
            return (await getAllDoneProjects(id, offset+20, projects))
        }
    }
    
    return projects
}


async function transformProjectData(dataArray, id){
    


    let handledData = []

    for (let data of dataArray){
        switch (true) {
            case data.path.includes('/johvi/div-01/piscine-js/') || data.path.includes('/johvi/div-01/piscine-js-2/') ||  data.path.includes('/johvi/div-01/piscine-js-3/'):
                data.from = "piscine-js"
                break;
            case data.path.includes('/johvi/div-01'):
                data.from = "div-01"
                break;
            case data.path.includes('/johvi/piscine-go') :
                data.from = "piscine-go"
                break;
            case data.path.includes('/johvi/onboarding'):
                data.from = "on-boarding"
                break;
            default:
                console.log(data)
                break;
        }

        data = await getProjectExtraInfo(data, id)
        handledData.push(data)
    }

    handledData.sort(function(a,b){
        return new Date(b.createdAt) - new Date(a.createdAt);
    });

    return addTotalXP(handledData)
}



async function getProjectExtraInfo(data, id){
    let objectId = data.objectId
    
    let projectExtraQuery = `{
        transaction (
          where: {
            path: {_eq: "${data.path}"},
            userId:  {_eq: ${id}}
          },
            limit: 1,
            order_by: {amount: desc_nulls_last}
        ){
            xp: amount
        }
        object (where: {id: {_eq: ${objectId}}}){
            name
        }
    }`

    let res = (await fetchDataWithQuery(projectExtraQuery)).data

    data.name = res.object[0].name
    if (res.transaction.length > 0){
        data.xp = res.transaction[0].xp
    } else {
        data.xp = 0
    }
    return data
}

function addTotalXP(dataArray){
    let goXP = 0
    let jsXP = 0
    let divXP = 0

    for (let i = dataArray.length-1; i >= 0; i--) {
        let data = dataArray[i] 
        switch (true) {
            case data.path.includes('/johvi/div-01/piscine-js/') || data.path.includes('/johvi/div-01/piscine-js-2/') ||  data.path.includes('/johvi/div-01/piscine-js-3/'):
                jsXP += data.xp
                data.totalXP =  jsXP
                break;
            case data.path.includes('/johvi/div-01'):
                divXP += data.xp
                data.totalXP =  divXP
                break;
            case data.path.includes('/johvi/piscine-go'):
                goXP += data.xp
                data.totalXP =  goXP
                break;
            case data.path.includes('/johvi/onboarding'):
                data.from = "on-boarding"
                break;
            default:
                console.log(data)
                break;
        }
    }
    return dataArray
}