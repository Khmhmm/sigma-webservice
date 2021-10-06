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
let form = document.querySelector("#fillForm");
form.style.display = "none";
let undermenu = document.querySelector("#undermenu");
undermenu.style.display = "none";
let pushDiv = document.querySelector(".push");
pushDiv.style.display = "none";

function pushNotification(text, color) {
    push.innerText = text;
    push.style.backgroundColor = color;
    push.style.display = "flow-root";
    setTimeout(
        ()=>{push.style.display = "none"}, 2000
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

let xhr_have_rights = new XMLHttpRequest();
xhr_have_rights.onreadystatechange = async function() {
    if (this.readyState == 4 && this.status == 200) {
        const json = Number(JSON.parse(xhr_have_rights.responseText));
        if (json>1) {
            addMenuButton.style.color = "white";
            addMenuButton.disabled = false;
            // TODO: СРОЧНО ЗАМЕНИТЬ НА НОВЫЙ ЗАКАЗ !!!!!!!!!!!!!!!!!!!!!
            form.innerHTML = `<div class="formHeader"> Новая типография </div>
                <div> Название <input class="formInput" id="formName"></input></div>
                <div> Адрес    <input class="formInput" id="formAddress"></input></div>
                <div> Телефон  <input class="formInput" id="formPhone" ></input></div>
                <p> Если все верно </p>`;
            rightsResult = true;

            newOrderButton = document.createElement("button");
            newOrderButton.classList.add("menuButton");
            newOrderButton.classList.add("formButton");
            newOrderButton.innerText="Отправить";
            newOrderButton.onclick = function() {
                let xhr = new XMLHttpRequest();
                xhr.onreadystatechange = function() {
                    if (this.readyState == 4 && this.status == 200) {
                        pushNotification("Запрос успешно отправлен", "green");
                    } else {
                        pushNotification("Сервер не распознал ваш запрос", "red");
                    }
                }
                // ПЕРЕПИСАТЬ РОУТЫ!!!!!!!!
                xhr.open("POST","/api/newTypography");
                xhr.setRequestHeader('Content-Type', 'application/json');
                xhr.send(JSON.stringify({'name':document.getElementById('formName').value,'address':document.getElementById('formAddress').value,'phone':document.getElementById('formPhone').value}));
            }
            form.appendChild(newOrderButton);
            // ПЕРЕПИСАТЬ РОУТЫ!!!!!!!!
        } else {
            rightsResult = false;
        }
    }
}
xhr_have_rights.open("GET", "/api/haveRights", true);
xhr_have_rights.send();

// block on while not xhr have rights is responsed

const haveRights = lockOnRights().then((res)=>res).catch((err)=>console.error(err));
let newOrderButton = null;

addMenuButton.onclick = function() {
    if(haveRights) {
        form.style.display = "flow-root";
        ordersList.style.display="none";
        undermenu.style.display="none";
    }
}

listMenuButton.onclick = () => {
    form.style.display = "none";
    ordersList.style.display="flow-root";
    undermenu.style.display="none";
}

handleMenuButton.onclick = () => {
    form.style.display="none";
    ordersList.style.display="none";
    undermenu.style.display="flow-root";
}
