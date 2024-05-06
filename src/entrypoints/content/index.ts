chrome.runtime.onMessage.addListener((_message, _sender, sendResponse) => {
    const innerText = document.body.innerText;
    sendResponse({innerText});
})