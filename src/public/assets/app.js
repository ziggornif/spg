import markdownIt from 'https://cdn.jsdelivr.net/npm/markdown-it@14.1.0/+esm';

async function postPrompt(theme) {
  const resp = await fetch("/prompt", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      theme,
    })
  })

  if (!resp.ok) {
    throw new Error('Network response was not ok');
  }

  return resp.body.getReader();
}

window.onload = async function () {
  const promptForm = document.getElementById("generator");

  promptForm.addEventListener('click', async (event) => {
    event.preventDefault();
    const promptElem = document.getElementById("query");
    const messagesDiv = document.getElementById("messages");
    const robotPrompt = document.createElement('p');
    const theme = document.querySelector('input[name="theme"]:checked')?.value;
    promptForm.disabled = true;
    const reader = await postPrompt(theme).catch(error => {
      console.log(error)
      return;
    });
    robotPrompt.innerHTML = "";
    messagesDiv.appendChild(robotPrompt);
    const decoder = new TextDecoder('utf-8');  
    // Read chunks of data from the stream
    let text = "![avatar](assets/img/avatar.png) ";
    const readStream = () => {
      reader.read().then(({ done, value }) => {
        if (done) {
          messagesDiv.appendChild(document.createElement("hr"));
          promptForm.disabled = false;
          return;
        }
        const chunkString = decoder.decode(value);
        text += chunkString;
        robotPrompt.innerHTML = markdownIt().render(text);
        promptElem.scrollIntoView();
        readStream();
      }).catch(error => {
          console.error('Error reading stream:', error);
      });
    };
    readStream();
  })
}