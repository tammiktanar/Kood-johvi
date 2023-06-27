function startSums(nr, curSum, start, res, resOutput) {
    if (curSum == nr) {
        resOutput.push(res.slice());
    }

    for (let i = start; i < nr; i++) {
        let tempSum = curSum + i;
        if (tempSum <= nr) {
            res.push(i);
            startSums(nr, tempSum, i, res, resOutput);
            res.pop();
        } else {
            return undefined
        }
    }
};

function sums(nr) {
    if (nr != 0) {
        let resOutput = [];
        let res = [];
        startSums(nr, 0, 1, res, resOutput);
        return resOutput;
    } else {
        return []
    }
};
