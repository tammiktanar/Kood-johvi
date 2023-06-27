import { getDateFormat, getXPFormatStr } from "./format.js"
import { fetchDataWithQuery } from "./query.js"


async function getSkillsDataFromTransactions(id = 3886, offset = 0, audits = []){
    let query = `{
        transaction(
          where: {
              userId: {_eq: ${id}},
              type: {_iregex: "(skill_)"},
          },
          offset: ${offset},
          limit: 10
        ){
          amount
          type
          userId
          createdAt
        }
    }`
    
    let returned_audits = (await fetchDataWithQuery(query)).data.transaction

    if (returned_audits.length > 0) {
        audits = audits.concat(returned_audits)
        return (await getSkillsDataFromTransactions(id, offset+10, audits))
    }

    return audits
}

export async function setSkills(id){
    let data = await getSkillsDataFromTransactions(id, 0, [])

    makeSkillsGraph(data)
}

function makeSkillsGraph(dataArray){
    setSkillsGraph(dataArray)
}


function setSkillsGraph(dataArray){
    let setData = {}
    let skills = []
    let skillsAmount = []

    dataArray.forEach(data => {
        if (setData[data.type] == undefined) setData[data.type] = 0
        setData[data.type] += data.amount
    });


    let skillsList = Object.keys(setData)

    skillsList.forEach(skill => {
        skillsAmount.push(setData[skill])
        skills.push(skill.split('skill_')[1].toUpperCase())
    });

    const ctx = document.getElementById('skills-graph').getContext('2d');
    const myChart = new Chart(ctx, {
        type: 'radar',
        data: {
            labels: skills,
            datasets: [{
                label: 'Skill amount',
                data: skillsAmount,
                backgroundColor: ['rgba(13, 110, 253, 0.5)'],
                fill: true,
                borderWidth: 3,
            }]
        },
        options: {

        }
    });
}