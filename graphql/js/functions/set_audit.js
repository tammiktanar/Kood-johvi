import { getDateFormat, getXPFormatStr } from "./format.js"
import { fetchDataWithQuery } from "./query.js"


async function getDataFromTransactions(id = 3886, offset = 0, audits = [], dataRequest = 'up'){
    let query = `{
        transaction(
          where: {
              userId: {_eq: ${id}},
              type: {_eq: "${dataRequest}"},
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
        return (await getDataFromTransactions(id, offset+10, audits, dataRequest))
    }

    return audits
}

export async function setAudit(id){
    let upData = await getDataFromTransactions(id, 0, [], 'up')
    let downData = await getDataFromTransactions(id, 0, [], 'down')

    makeAuditGraph(upData, downData)
}

function makeAuditGraph(upData, downData){
    let auditData = upData.concat(downData)

    auditData.sort(function(a,b){
        return new Date(a.createdAt) - new Date(b.createdAt);
    });

    setAuditProgressBar(upData, downData)
    setAuditGraph(auditData)
}

function setAuditProgressBar(upData, downData){
    let up = upData.reduce((acc, transaction) => acc + transaction["amount"], 0)
    let down = downData.reduce((acc, transaction) => acc + transaction["amount"], 0)

    let downBar = document.getElementById('audit-down-rate')
    downBar.style.width = ((down/up)*100).toFixed(2) + "%"
    downBar.innerText = getXPFormatStr(down)

    let upBar = document.getElementById('audit-up-rate')
    upBar.style.width = ((up/down)*100).toFixed(2) + "%"
    upBar.innerText = getXPFormatStr(up)

    let auditRatioNr = document.getElementById('audit-ratio')
    auditRatioNr.innerText = "Audits ratio: " + (up/down).toFixed(2)
}

function setAuditGraph(dataArray){
    let dates = []
    let datesData = []

    let up = 0
    let down = 0
    dataArray.forEach(data => {
        if (data.type == "down"){
            down += data.amount
        } else {
            up += data.amount
        }

        let tempNr = up / down

        if (tempNr < 0) tempNr = 0

        dates.push(( new Date(data.createdAt)))
        datesData.push(tempNr)
    });


    const upCTX = (ctx, value) => ctx.p0.parsed.y < ctx.p1.parsed.y ? value : undefined
    const downCTX = (ctx, value) => ctx.p0.parsed.y > ctx.p1.parsed.y ? value : undefined




    const ctx = document.getElementById('audit-graph').getContext('2d');
    const myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: dates,
            datasets: [{
                label: 'Audit ratio over time',
                data: datesData,
                backgroundColor: ['rgb(13 110 253)'],
                fill: false,
                borderWidth: 3,
                segment: {
                    borderColor: ctx => upCTX(ctx, 'rgb(25 135 84)') || downCTX(ctx, 'rgb(220 53 69)'),
                    backgroundColor: ctx => upCTX(ctx, 'rgb(25 135 84)') || downCTX(ctx, 'rgb(220 53 69)'),
                }
            }]
        },
        options: {
            scales: {
                y: {
                    beginAtZero: true
                },

                x: {
                    type: 'time',
                    time: {
                        unit: 'month'
                    },
                }
            }
        }
    });
}