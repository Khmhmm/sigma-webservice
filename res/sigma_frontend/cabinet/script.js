/* elements example:

/ ** undermenu example ** /
<div class="undermenu" id="undermenu">
        <!-- Не забыть вынести в js!! -->
        <button type="button" class="menuButton" id="addButton">Добавить</button>
        <button type="button" class="menuButton" id="viewButton">Просмотр</button>
        <button type="button" class="menuButton" id="editButton">Изменить</button>
        <button type="button" class="menuButton" id="accountButton">Аккаунт</button>
</div>

<div class="norights" id="norights">У вас нет прав на просмотр этой страницы</div>

/ ** active orders list ** /
<ul class="ordersList" id="activeOrders">
    <li class="activeOrder">
        <p class="orderInfo">Заказ №1
        <span class="orderInfo">Пушкин "Капитанская дочка"</span></p>
        <p class="orderInfo">Заказчик: СОШ №1
        <span class="orderInfo">Типография: Лиговка </span></p>
        <p class="orderInfo">Тип: книга
        <span class="orderInfo">Статус: Не готово </span> </p>
    </li>
</ul>

/ ** form for sending ** /
<form action="" method="">
    <div class="formHeader"> Новая типография </div>
    <div> Название <input class="formInput" name="name"></input></div>
    <div> Адрес    <input class="formInput" name="address"></input></div>
    <div> Телефон  <input class="formInput" type="phone" name="phone"></input></div>
    <p> Если все верно </p>
    <p> <input class="menuButton formButton" type="submit"></input> </p>
</form>
*/

let loadingStatus = document.querySelector("#loadingStatus");
let ordersList = document.querySelector("#activeOrders");

// forms which will be switched by buttons on SPA
let form = document.querySelector("#fillForm");
form.style.display = "none";
let typoForm = document.createElement("div");
typoForm.id = "typographyForm";
let orderForm = document.createElement("div");
orderForm.id = "orderForm";

let undermenu = document.querySelector("#undermenu");
undermenu.style.display = "none";
let pushDiv = document.querySelector(".push");
pushDiv.style.display = "none";

function pushNotification(text, color) {
    pushDiv.innerText = text;
    pushDiv.style.backgroundColor = color;
    pushDiv.style.display = "flow-root";
    setTimeout(
        ()=>{pushDiv.style.display = "none"}, 2000
    );
}

let xhr_active_orders = new XMLHttpRequest();
xhr_active_orders.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
       loadingStatus.innerText = "Список загружен";
       const json = JSON.parse(xhr_active_orders.responseText);
       const _promise = Promise.all(json.map(async (el) => {
           let new_li = document.createElement('li');
           new_li.classList.add('activeOrder');
           let orderStatus = 'Создано';
           if (el['status'] == 2) {
               orderStatus = 'Обрабатывается';
           } else if (el['status'] == 3) {
               orderStatus = 'Готово';
           }

           new_li.innerHTML = '<p class="orderInfo"> Заказ<span class="orderInfo">' + el['author'] + '"' + el['ord'] + '"' + '</span></p>'
            + '<p class="orderInfo">Заказчик: ' + el['ordermaker'] + '<span class="orderInfo">Типография: ' + el['typography'] + '</span></p>'
            + '<p class="orderInfo">Тип: ' + el['ty'] + '<span class="orderInfo">Статус: ' + orderStatus + '</span></p>';

            ordersList.appendChild(new_li);
       }));

   } else {
       loadingStatus.innerText = "Ошибка получения списка: " + this.status;
   }
};
xhr_active_orders.open("GET", "/api/activeOrders", true);
xhr_active_orders.send();

let addMenuButton = document.querySelector("#addMenuButton");
addMenuButton.disabled = true;
let listMenuButton = document.querySelector("#listMenuButton");
let handleMenuButton = document.querySelector("#handleMenuButton");

let rightsResult = null;
let lockOnRights = function f() {
    return new Promise(resolve => setTimeout(()=>{
        if (rightsResult!=null){
            resolve(rightsResult);
        } else {
            reject(null);
        }
    }, 100));
}

const new_typography_onclick = function() {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            pushNotification("Запрос о добавлении типографии успешно отправлен", "green");
        } else if (this.readyState != 4) {
            pushNotification("Запрос обрабатывается", "yellow");
        } else {
            pushNotification("Произошла ошибка. Ответ сервера:" + this.status, "red");
        }
    }
    xhr.open("POST","/api/newTypography");
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.send(JSON.stringify({'name':document.getElementById('formName').value,'address':document.getElementById('formAddress').value,'phone':document.getElementById('formPhone').value}));
}

const new_order_onclick = function() {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            pushNotification("Запрос о добавлении типографии успешно отправлен", "green");
        } else if (this.readyState != 4) {
            pushNotification("Запрос обрабатывается", "yellow");
        } else {
            pushNotification("Произошла ошибка. Ответ сервера:" + this.status, "red");
        }
    }

    xhr.open("POST","/api/newOrder");
    xhr.setRequestHeader('Content-Type', 'application/json');

    xhr.send(JSON.stringify(
        {'name':document.querySelector('#orderForm > div > input.formName').value,
        'author_id':Number(document.querySelector('#orderForm > div > p > input.formAuthor').value),
        'category_id':Number(document.querySelector('#orderForm > div > p > input.formCategory').value),
        'year':Number(document.querySelector('#orderForm > div > input.formYear').value),
        'type_id':Number(document.querySelector('#orderForm > div > p > input.formType').value),
        'typography_id':Number(document.querySelector('#orderForm > div > p > input.formTypography').value),
        'ordermaker_id':Number(document.querySelector('#orderForm > div > p > input.formOrdermaker').value),
        'price':Number(document.querySelector('#orderForm > div > input.formPrice').value)}
    ));
}

let xhr_have_rights = new XMLHttpRequest();
xhr_have_rights.onreadystatechange = async function() {
    if (this.readyState == 4 && this.status == 200) {
        const json = Number(JSON.parse(xhr_have_rights.responseText));
        if (json>1) {
            addMenuButton.style.color = "white";
            addMenuButton.disabled = false;
            rightsResult = true;

            let newButton = document.createElement("button");
            newButton.classList.add("menuButton");
            newButton.classList.add("formButton");
            newButton.innerText="Отправить";

            let newTypographyButton = newButton;
            newTypographyButton.onclick = new_typography_onclick;

            let newOrderButton = newButton;
            newOrderButton.onclick = new_order_onclick;


            typoForm.appendChild(newTypographyButton);
            orderForm.appendChild(newOrderButton);
        } else {
            rightsResult = false;
        }
    }
}
xhr_have_rights.open("GET", "/api/haveRights", true);
xhr_have_rights.send();

// block on while not xhr have rights is responsed

const haveRights = lockOnRights().then((res)=>res).catch((err)=>console.error(err));

listMenuButton.onclick = () => {
    if(form.childNodes.length > 0) {
        form.removeChild(form.childNodes[0]);
    }
    form.style.display = "none";
    ordersList.style.display="flow-root";
    undermenu.style.display="none";
}

handleMenuButton.onclick = () => {
    if(form.childNodes.length > 0) {
        form.removeChild(form.childNodes[0]);
    }
    form.style.display="none";
    ordersList.style.display="none";
    undermenu.style.display="flow-root";
}

async function getData(xhr, route, colName, className) {
    collection = [];
    xhr.onreadystatechange = async function() {
        if (this.readyState == 4 && this.status == 200) {
            collection = JSON.parse(this.responseText);
            let new_div = document.createElement('div');
            new_div.classList.add('formInput');
            new_div.innerText = colName;
            let new_p = document.createElement('p');
            for (let i=0;i<collection.length;i++) {
                el = collection[i];
                new_p.innerHTML += `<input class="formInput `+className+`" name="`+className+`" type="radio" value="`+el['id']+`">`+ el['name'];
            }
            new_div.appendChild(new_p);
            orderForm.appendChild(new_div);
        }
    }
    xhr.open("GET", route, true);
    xhr.send();
    await xhr.onreadystatechange();
}

orderForm.innerHTML += `<div class="formHeader"> Новый заказ </div>`;
orderForm.innerHTML += `<div> Название <input class="formInput formName"></input></div>`;
orderForm.innerHTML += `<div> Год <input class="formInput formYear"></input></div>`;
orderForm.innerHTML += `<div> Цена <input class="formInput formPrice"></input></div>`;


let xhr_authors = new XMLHttpRequest();
getData(xhr_authors, "/api/getAuthors", "Автор", "formAuthor");

let xhr_categories = new XMLHttpRequest();
categories = getData(xhr_categories, "/api/getCategories", "Категория", "formCategory");

let xhr_types = new XMLHttpRequest();
getData(xhr_types, "/api/getTypes", "Тип издания", "formType");

let xhr_typography = new XMLHttpRequest();
getData(xhr_typography, "/api/getTypographies", "Типография", "formTypography");

let xhr_ordermakers = new XMLHttpRequest();
getData(xhr_ordermakers, "/api/getOrdermakers", "Заказчик", "formOrdermaker");


typoForm.innerHTML = `<div class="formHeader"> Новая типография </div>
        <div> Название <input class="formInput" id="formName"></input></div>
        <div> Адрес    <input class="formInput" id="formAddress"></input></div>
        <div> Телефон  <input class="formInput" id="formPhone" ></input></div>`;



addMenuButton.onclick = async function() {
    if(haveRights) {
        if(form.childNodes.length > 0) {
            form.removeChild(form.childNodes[0]);
        }
        form.style.display = "flow-root";
        form.appendChild(orderForm);
        ordersList.style.display="none";
        undermenu.style.display="none";
    }
}
