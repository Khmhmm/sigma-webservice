let signupButton = document.getElementById('signup');
let loginButton = document.getElementById('login');

let user = "";
let pass = "";

let xhttp = new XMLHttpRequest();
xhttp.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
       // Typical action to be performed when the document is ready:
       // document.getElementById("demo").innerHTML = xhttp.responseText;
       // document.cookie = "user="+user+";pass="+pass+";SameSite=Lax;Secure";
       alert(xhttp.responseText)
    }
};

function sendXhr(xhr) {
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