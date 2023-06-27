const ctx = {
  multiInteractions: {
    affirmations: {
      i_am_fine: {
        affirmation: "i am fine",
        responses: ["cool", "awesome"],
      },
    },
    orders: {
      turn_on_the_lights: {
        order: "turn on the lights!",
        responses: ["done"],
      },
    },
    questions: {
      how_are_you: {
        question: "how are you?",
        responses: ["well thanks, and you?"],
      },
    },
  },
  multipleTypes: [
    "Questions: How did Bob Marley die? - Response: Nesta Robert Bob Marley, OM was a Jamaican singer-songwriter and musician.",
    "Questions: How did Bob Marley die? - Response: He was the rhythm guitarist and lead singer for the ska , rocksteady and reggae bands The Wailers and Bob Marley & The Wailers (1974–1981).",
    "Questions: how did crater lake get its color? - Response: Crater Lake is a caldera lake located in the south-central region of the U.S. state of Oregon .",
    "Questions: how did crater lake get its color? - Response: It is the main feature of Crater Lake National Park and famous for its deep blue color and water clarity .",
    "Questions: how much is a ream of paper? - Response: Various measures of paper quantity have been and are in use.",
    "Questions: how much is a ream of paper? - Response: Although there are no S.I. units such as quires and bales , there are ISO and DIN standards for the ream.",
    "Questions: how much is a ream of paper? - Response: Expressions used here include U.S. Customary units.",
    "Orders: shutdown! - Response: no!",
    "Orders: shutdown! - Response: Yes Sr!",
    "Orders: shutdown! - Response: goodbye!!",
    "orders: Quote something! - Response: Make improvements, not excuses. Seek respect, not attention.",
    "orders: Quote something! - Response: Life becomes easier and more beautiful when we can see the good in other people.",
    "orders: Quote something! - Response: Pursue what catches your heart, not what catches your eyes.",
  ],
  multResult: {
    questions: {
      how_did_bob_marley_die: {
        question: "How did Bob Marley die?",
        responses: [
          "Nesta Robert Bob Marley, OM was a Jamaican singer-songwriter and musician.",
          "He was the rhythm guitarist and lead singer for the ska , rocksteady and reggae bands The Wailers and Bob Marley & The Wailers (1974–1981).",
        ],
      },
      how_did_crater_lake_get_its_color: {
        question: "how did crater lake get its color?",
        responses: [
          "Crater Lake is a caldera lake located in the south-central region of the U.S. state of Oregon .",
          "It is the main feature of Crater Lake National Park and famous for its deep blue color and water clarity .",
        ],
      },
      how_much_is_a_ream_of_paper: {
        question: "how much is a ream of paper?",
        responses: [
          "Various measures of paper quantity have been and are in use.",
          "Although there are no S.I. units such as quires and bales , there are ISO and DIN standards for the ream.",
          "Expressions used here include U.S. Customary units.",
        ],
      },
    },
    orders: {
      shutdown: {
        order: "shutdown!",
        responses: ["no!", "Yes Sr!", "goodbye!!"],
      },
      quote_something: {
        order: "Quote something!",
        responses: [
          "Make improvements, not excuses. Seek respect, not attention.",
          "Life becomes easier and more beautiful when we can see the good in other people.",
          "Pursue what catches your heart, not what catches your eyes.",
        ],
      },
    },
  },
};

const toProp = (str) => {
  str = str.toLowerCase();
  if (str[str.length - 1] === "?" || str[str.length - 1] === "!")
    str = str.substr(0, str.length - 1);
  str = str.split(" ").join("_");
  return str;
};

const neuron = (arr) => {
  const res = {};

  arr.forEach((el) => {
    let firstType = el.split(": ")[0];
    let firstItem = el.split(": ")[1].split(" - ")[0];
    let secondItem = el.split(" - ")[1].split(": ")[1];

    let questions = firstType.toLowerCase();
    let property = toProp(firstItem);
    let question = questions.substr(0, questions.length - 1);
    let questionItem = firstItem;
    let responses = "responses";
    let response = secondItem;

    const tmp = {};
    tmp[questions] = res[questions] ? res[questions] : {};
    let prevResponses = [];
    if (res[questions]) {
      if (res[questions][property]) {
        prevResponses.push(...res[questions][property][responses]);
      }
    }
    tmp[questions][property] = {};
    tmp[questions][property][question] = questionItem;
    prevResponses.push(response);
    tmp[questions][property][responses] = prevResponses;
    res[questions] = tmp[questions];
  });
  return res;
};
