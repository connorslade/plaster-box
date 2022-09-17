let data = {
  submit: () => {
    document.querySelector("#submit").classList.add("is-loading");
    let data = document.querySelector("#form").elements;

    if (data.text.value === "") {
      err("No Body", 3000);
      return;
    }

    localStorage.removeItem("text");

    fetch("/new", {
      method: "post",
      body: data.text.value,
      headers: {
        Name: encodeURIComponent(data.name.value ?? "Unnamed Box"),
        "Content-Type": "text/plain; charset=UTF-16",
      },
    })
      .then(async (d) => {
        const text = await d.text();
        if (d.ok) window.location.pathname = `/b/${text}`;
        else err(text);
      })
      .catch(err);
  },
};

function err(e, time) {
  document.querySelector("#submit").classList.remove("is-loading");
  bulmaToast.toast({
    message: e.toString(),
    duration: time || 5000,
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
