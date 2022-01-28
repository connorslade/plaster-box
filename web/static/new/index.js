let data = {
  submit: () => {
    let data = document.querySelector("#form").elements.text.value;

    fetch("/new", {
      method: "post",
      body: data,
    })
      .then((d) => d.text())
      .then((d) => (window.location.pathname = `/b/${d}`));
  },
};
