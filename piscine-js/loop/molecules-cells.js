const DNA_TO_RNA = {
    "G": "C",
    "C": "G",
    "T": "A",
    "A": "U"
}

const RNA_TO_DNA = {
    "C": "G",
    "G": "C",
    "A": "T",
    "U": "A"
}

function RNA(DNA){
    let res = ""
    for (let char of DNA){
        res += DNA_TO_RNA[char]
    }

    return res
}

function DNA(RNA){
    let res = ""
    for (let char of RNA){
        res += RNA_TO_DNA[char]
    }

    return res
}