let data = {
  submit: () => {
    document.querySelector("#submit").classList.add("is-loading");
    let data = document.querySelector("#form").elements;
    localStorage.removeItem("text");

    fetch("/new", {
      method: "post",
      body: data.text.value,
      headers: { Name: data.name.value || "Unnamed Box" },
    })
      .then((d) => d.text())
      .then((d) => (window.location.pathname = `/b/${d}`))
      .catch(err);
  },
};

function err(e) {
  document.querySelector("#submit").classList.remove("is-loading");
  bulmaToast.toast({
    message: e.toString(),
    duration: 10000,
    type: "is-danger",
    dismissible: true,
    animate: { in: "fadeIn", out: "fadeOut" },
  });
}

window.addEventListener("load", () => {
  const textBox = document.querySelector(".text-input");
  textBox.value = localStorage.getItem("text");

  textBox.style.height = `${textBox.scrollHeight}px`;
  textBox.style.overflowY = "hidden";

  textBox.addEventListener("input", (e) => {
    localStorage.setItem("text", e.target.value);
    textBox.style.height = "auto";
    textBox.style.height = textBox.scrollHeight + "px";
  });
});
