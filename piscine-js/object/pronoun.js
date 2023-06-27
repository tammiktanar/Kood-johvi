const pronouns = ["i", "you", "he", "she", "it", "we", "they"];
const pronoun = (str) => {
  let newObj = {};
  str.split(/[\s.,]+/).forEach((word, index, array) => {
    word = word.toLowerCase();
    if (!pronouns.includes(word)) {
      return;
    }

    let next = array[index + 1];

    if (next && pronouns.includes(next.toLowerCase())) {
      next = undefined;
    }

    if (!newObj[word]) {
      newObj[word] = {
        word: [],
        count: 1,
      };
    } else {
      newObj[word].count++;
    }

    if (next) {
      newObj[word].word.push(array[index + 1]);
    }
  });

  return newObj;
};
