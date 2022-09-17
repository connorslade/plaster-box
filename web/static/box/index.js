let data = {
  rawRedirect: () => {
    let id = window.location.pathname.split("/")[2];
    window.location.pathname = `/raw/${id}`;
  },
  copy: () => {
    let data = document.querySelector("#data").innerText;
    data = data.replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">");
    navigator.clipboard.writeText(data);
  },
};
