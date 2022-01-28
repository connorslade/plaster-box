let data = {
  submit: () => {
    document.querySelector("#submit").classList.add("is-loading");
    let data = document.querySelector("#form").elements;
    localStorage.removeItem("text");

    fetch("/new", {
      method: "post",
      body: data.text.value,
      headers: {Name: data.name.value},
    })
      .then((d) => d.text())
      .then((d) => (window.location.pathname = `/b/${d}`))
      .catch(err);
  },
};

window.addEventListener("load", () => {
  const textBox = document.querySelector(".text-input");

  textBox.value = localStorage.getItem("text");

  textBox.addEventListener("input", (e) => {
    localStorage.setItem("text", e.target.value);
  });
});
