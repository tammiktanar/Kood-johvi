import { getDateFormat, getXPFormatStr } from "./format.js";
import { manageOnClick } from "./onClick.js";

export function setGraphs(data){
    setGraph(data, 'piscine-go', "go-graph")
    setGraph(data, 'div-01', "div-graph")
    setGraph(data, 'piscine-js', "js-graph")
}

export function setGraph(dataArray, graphType, where){
    let setData = []
    let dates = []
    let datesData = []

    if (!where || !dataArray || !graphType) return
    
    dataArray.forEach(data => {
        if (data.from == graphType){
            setData.push({
                date: data.createdAt,
                amount: data.totalXP
            })
        }
    });

    setData.sort(function(a,b){
        return new Date(a.date) - new Date(b.date);
    });

    setData.forEach(data => {
        dates.push(( new Date(data.date)))
        datesData.push(data.amount)
    })


    const ctx = document.getElementById(where).getContext('2d');
    const myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: dates,
            datasets: [{
                label: 'xp over time',
                data: datesData,
                backgroundColor: ['rgb(13 110 253)'],
                borderColor: ['rgb(13 110 253)'],
                borderWidth: 1
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

