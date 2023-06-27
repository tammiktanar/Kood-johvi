function dogYears(planet, age){
    let res = age / 31557600
    switch (planet.toLowerCase()) {
        case "mercury":
            res = res / 0.2408467
            break;

        case "venus":
            res = res / 0.61519726
            break;

        case "mars":
            res = res / 1.8808158
            break;
            
        case "jupiter":
            res = res / 11.862615
            break;
            
        case "saturn":
            res = res / 29.447498
            break;
            
        case "uranus":
            res = res / 84.016846
            break;
            
        case "neptune":
            res = res / 164.79132
            break;
    
        default:
            break;
    }
    return parseInt((res * 7).toFixed(2)*100)/100
}

/*
    earth : orbital period 1.0 Earth years, 365.25 Earth days, or 31,557,600 seconds
    mercury : orbital period 0.2408467 Earth years
    venus : orbital period 0.61519726 Earth years
    mars : orbital period 1.8808158 Earth years
    jupiter : orbital period 11.862615 Earth years
    saturn : orbital period 29.447498 Earth years
    uranus : orbital period 84.016846 Earth years
    neptune : orbital period 164.79132 Earth years
    So if you were told someone that their dog were 1,000,000,
*/