(function () {
    let [formToCreateEdit] = document.forms;
    let inputPws = formToCreateEdit.querySelectorAll(".pws .pw");

    formToCreateEdit.addEventListener("submit", async function (e) {
        e.preventDefault();

        e = new FormData(formToCreateEdit);
        e.append("pw_for_logs", ""); // NULL
        e.append("pw_for_deletion", ""); // NULL
        let response = await fetch("?page-edited", {
            method: "POST",
            headers: {"Content-Type": "application/json",},
            body: JSON.stringify(Object.fromEntries(e.entries())),
        }).then(r => r.json());
        console.table(response);
    });
}) ();