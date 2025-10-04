(function () {
    let [formToCreateEdit] = document.forms;
    let checkboxEnforcingPwMatch = formToCreateEdit.querySelector(".pws input.enforce-match");
    let inputPws = formToCreateEdit.querySelectorAll(".pws .pw");

    function enforcePwMatch(inputEl) {
        inputPws.forEach(function (el) {
            el.value = inputEl.value;
        });
    }
    function listenToMatchPws(inputEl) {
        inputEl.oninput = (checkboxEnforcingPwMatch.checked) ? function () {
            enforcePwMatch(inputEl);
        } : null;
    }
    inputPws.forEach(listenToMatchPws);
    checkboxEnforcingPwMatch.addEventListener("change", function () {
        inputPws.forEach(listenToMatchPws);
    });

    formToCreateEdit.addEventListener("submit", async function (e) {
        e.preventDefault();

        e = new FormData(formToCreateEdit);
        let response = await fetch("?page-created", {
            method: "POST",
            headers: {"Content-Type": "application/json",},
            body: JSON.stringify(Object.fromEntries(e.entries())),
        })
            .then(r => r.json())
            .then(r => r ? window.location.replace(window.location.pathname) : undefined);
        console.table(response);
    });
}) ();