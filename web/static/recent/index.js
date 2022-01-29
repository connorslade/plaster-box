document.querySelectorAll('tbody tr').forEach(e => {
  let id = e.attributes['id'].value;
  e.addEventListener('click', () => {
    window.location.pathname = `/b/${id}`;
  })
})
