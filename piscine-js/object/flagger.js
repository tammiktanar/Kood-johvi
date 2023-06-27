const flags = (obj) => {
  const res = {};
  const alias = {};
  alias.h = "help";
  let last;
  for (let prop in obj)
    if (prop !== "help") {
      let short = prop[0];
      alias[short] = prop;
      last = prop;
    }
  res.alias = alias;
  let description = "";
  if (obj.help)
    for (let i = 0; i < obj.help.length; i++) {
      let el = obj.help[i];
      description += `-${el[0]}, --${el}: ${obj[el]}`;
      if (i < obj.help.length - 1) description += "\n";
    }
  else
    for (let el in obj)
      if (el !== "help") {
        description += `-${el[0]}, --${el}: ${obj[el]}`;
        if (el !== last) description += "\n";
      }

  res.description = description;
  return res;
};
