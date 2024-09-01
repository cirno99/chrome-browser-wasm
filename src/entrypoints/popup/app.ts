import van from "vanjs-core"
const { div, h1, input, button, ul, li } = van.tags

import init, { find } from "../../wasm/pkg";

const target_content = van.state("")
const query = van.state("")
const search_result = van.state<string[]>([])
const log_state = van.state("")

const ResultList = () => {
  return ul(search_result.val.map(it => li(it)))
}

const App = () => {
  const load = () => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      if (tabs[0].id === undefined) return;
      chrome.tabs.sendMessage<{}, { innerText: string }>(
        tabs[0].id,
        {},
        (response) => {
          target_content.val = response.innerText;
        }
      );
    });
  }
  load()

  const handleClick = async () => {
    await init();

    const result = find(target_content.val, query.val);
    search_result.val = result;
  };

  return div({ class: 'container py-4 py-sm-5' },
    div({},
      h1("Popup!"),
      div(log_state),
      div(
        input({ type: 'text', value: query, oninput: e => query.val = e.target.value, placeholder: 'to search' }),
        button({
          onclick: async () => await handleClick()
        }, 'search')
      ),
      () => ResultList()
    )
  )
}

export default App
