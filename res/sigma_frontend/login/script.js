let hat = document.querySelector('.hat');
hat.onclick = function() {
    window.location = "/";
}

let signupButton = document.getElementById('signup');
let loginButton = document.getElementById('login');
let warn = document.getElementById('warn');

let user = "";
let pass = "";

let xhttp = new XMLHttpRequest();
xhttp.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
       // Typical action to be performed when the document is ready:
       // document.getElementById("demo").innerHTML = xhttp.responseText;
       // document.cookie = "user="+user+";pass="+pass+";SameSite=Lax;Secure";
       document.cookie = "hash="+xhttp.responseText+";SameSite=Lax;Secure";
       window.location = '/cabinet';
   } else if(this.readyState == 4 && this.status != 200){
       warn.innerText = "Неверные данные";
   } else {
       signupButton.disabled = false;
       loginButton.disabled = false;
   }
};

function sendXhr(xhr) {
    warn.innerText = "";
    signupButton.disabled = true;
    loginButton.disabled = true;
    xhttp.setRequestHeader('Content-Type', 'application/json');
    user = document.getElementById('user').value;
    pass = document.getElementById('pass').value;
    xhttp.send(JSON.stringify({ 'u': user, 'p': pass }));
}

login.onclick = () => {
    xhttp.open("POST", "/api/login", true);
    sendXhr(xhttp);
}

signup.onclick = () => {
    xhttp.open("POST", "/api/signup", true);
    sendXhr(xhttp);
}
