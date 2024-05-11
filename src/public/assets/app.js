import markdownIt from 'https://cdn.jsdelivr.net/npm/markdown-it@14.1.0/+esm';

async function postPrompt(promptMessage) {
  const resp = await fetch("/prompt", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      question: promptMessage,
    })
  })

  if (!resp.ok) {
    throw new Error('Network response was not ok');
  }

  return resp.body.getReader();
}

function addLoader(element) {
  const loader = document.createElement('span');
  loader.setAttribute("aria-busy", "true")
  loader.id = "loader";
  loader.textContent = "🤖 is typing...";
  element.appendChild(loader)
}

function removeLoader(element) {
  const child = document.getElementById("loader");
  element.removeChild(child);
}

function preparePrompt(theme) {
  switch (theme) {
    case "ecology":
      return "Propose moi une nouvelle idée de projet sur le thème de l'écologie."
    case "3dprint":
      return "Propose moi une nouvelle idée de projet sur le thème de l'impression 3D."
    case "music":
      return "Propose moi une nouvelle idée de projet sur le thème de la musique."
    case "cooking":
      return "Propose moi une nouvelle idée de projet sur le thème de la cuisine."
    default:
      return "Propose moi une nouvelle idée de projet."
  }
}

window.onload = async function () {
  const promptForm = document.getElementById("generator");

  promptForm.addEventListener('click', async (event) => {
    event.preventDefault();
    const promptElem = document.getElementById("query");
    const messagesDiv = document.getElementById("messages");
    const robotPrompt = document.createElement('p');
    addLoader(messagesDiv);
    const prompt = preparePrompt(document.querySelector('input[name="theme"]:checked')?.value);
    console.log('Sent prompt :', prompt);
    const reader = await postPrompt(prompt).catch(error => {
      console.log(error)
      return;
    });
    removeLoader(messagesDiv);
    robotPrompt.innerHTML = "";
    messagesDiv.appendChild(robotPrompt);
    const decoder = new TextDecoder('utf-8');  
    // Read chunks of data from the stream
    let text = "![avatar](assets/img/avatar.png) ";
    const readStream = () => {
      reader.read().then(({ done, value }) => {
        if (done) {
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
    messagesDiv.appendChild(document.createElement("hr"));
  })
}