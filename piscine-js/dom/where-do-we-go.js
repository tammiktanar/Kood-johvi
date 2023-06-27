import { places } from "./where-do-we-go.data.js";

export const explore = () => {
  let p = sortPlaces(places);

  createHeading();
  createDirection();

  let i = Math.round(window.scrollY / window.innerHeight);
  let currentPlace;

  const setCurrentPlace = (index) => {
    currentPlace = p[index];
    refreshHeading(currentPlace);
  };

  setCurrentPlace(i);

  let lastPos = window.scrollY;
  window.addEventListener("scroll", (e) => {
    let index = Math.round(window.scrollY / window.innerHeight);
    setCurrentPlace(index);

    if (lastPos < scrollY) {
      document.querySelector(".direction").textContent = "S";
    } else {
      document.querySelector(".direction").textContent = "N";
    }

    lastPos = scrollY;
  });

  renderSections(p);
};

const createDirection = () => {
  let el = document.createElement("div");
  el.classList.add("direction");
  el.textContent = "S";
  document.body.append(el);
};

const refreshHeading = (p) => {
  let h = document.querySelector("a.location");
  h.textContent = `${p.name}\n${p.coordinates}`;
  h.style.color = p.color;
  h.setAttribute("href", `https://www.google.com/maps/place/${p.coordinates}`);
};

const createHeading = () => {
  let el = document.createElement("a");
  el.classList.add("location");
  el.setAttribute("target", "_blank");
  el.setAttribute("href", "#");
  document.body.append(el);
};

const renderSections = (p) => {
  p.forEach((q) => {
    document.body.innerHTML += `<section style="background: url('./where-do-we-go_images/${q.name
      .split(",")[0]
      .toLowerCase()
      .replace(/[ ]/g, "-")}.jpg');"></section>`;
  });
};

const sortPlaces = (p) =>
  p.sort((a, b) => {
    let [lta] = DMS(a.coordinates);
    let [ltb] = DMS(b.coordinates);

    console.log(lta, ltb);

    return ltb - lta;
  });

const DMS = (p) => {
  let [lat, long] = p.split(" ");

  let lt = DMShelper(lat.slice(0, -1));
  let ln = DMShelper(long.slice(0, -1));

  if (lat.slice(-1) === "S") lt = -lt;
  if (lat.slice(-1) === "W") ln = -ln;

  return [lt, ln];
};

const DMShelper = (lat) => {
  let deg = +lat.slice(0, lat.indexOf("°"));
  lat = lat.slice(lat.indexOf("°") + 1);

  let min = +lat.slice(0, lat.indexOf("'"));
  lat = lat.slice(lat.indexOf("'") + 1);
  let sec = +lat.slice(0, -1);

  return deg + min / 60 + sec / 60 / 60;
};