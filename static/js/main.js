// Tell HTMX to swap the content even if the response is a `400 Bad Request`.
window.onload = function () {
    document.body.addEventListener('htmx:beforeSwap', function (evt) {
        if (evt.detail.xhr.status === 400) {
            evt.detail.shouldSwap = true;
        }
    });
};