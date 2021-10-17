let hat = document.querySelector('.hat');
hat.onclick = function() {
    window.location = "/";
}

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
let authorForm = document.createElement("div");
authorForm.id = "authorForm";
let ordermakerForm = document.createElement("div");
ordermakerForm.id = "ordermakerForm";

let addButton = document.querySelector('#addButton');
let viewButton = document.querySelector('#viewButton');
let editButton = document.querySelector('#editButton');
let accountButton = document.querySelector('#accountButton');

let undermenu = document.querySelector("#undermenu");
undermenu.style.display = "none";
let pushDiv = document.querySelector(".push");
pushDiv.style.display = "none";

let journal = document.createElement("div");
journal.id = "journal";
journal.classList.add("activeOrder");
journal.style.display = "none";
document.body.appendChild(journal);

function pushNotification(text, color) {
    pushDiv.innerText = text;
    pushDiv.style.backgroundColor = color;
    pushDiv.style.borderColor = color;
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
};

const new_typography_onclick = function() {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            pushNotification("Запрос о добавлении типографии успешно отправлен", "72D38C");
        } else if (this.readyState != 4) {
            pushNotification("Запрос о добавлении типографии обрабатывается", "88ACFF");
        } else {
            pushNotification("Произошла ошибка. Ответ сервера:" + this.status, "D17373");
        }
    }
    xhr.open("POST","/api/newTypography");
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.send(JSON.stringify({'name':document.getElementById('formName').value,'address':document.getElementById('formAddress').value,'phone':document.getElementById('formPhone').value}));
};

const new_order_onclick = function() {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            pushNotification("Запрос о добавлении заказа успешно отправлен", "72D38C");
        } else if (this.readyState != 4) {
            pushNotification("Запрос о добавлении заказа обрабатывается", "88ACFF");
        } else {
            pushNotification("Произошла ошибка. Ответ сервера:" + this.status, "D17373");
        }
    }

    xhr.open("POST","/api/newOrder");
    xhr.setRequestHeader('Content-Type', 'application/json');

    xhr.send(JSON.stringify(
        {'name':document.querySelector('#orderForm > div > input.formName').value,
        'author_id':Number(document.querySelector('#orderForm > div > p > input.formAuthor:checked').value),
        'category_id':Number(document.querySelector('#orderForm > div > p > input.formCategory:checked').value),
        'year':Number(document.querySelector('#orderForm > div > input.formYear').value),
        'type_id':Number(document.querySelector('#orderForm > div > p > input.formType:checked').value),
        'typography_id':Number(document.querySelector('#orderForm > div > p > input.formTypography:checked').value),
        'ordermaker_id':Number(document.querySelector('#orderForm > div > p > input.formOrdermaker:checked').value),
        'price':Number(document.querySelector('#orderForm > div > input.formPrice').value)}
    ));
};

const new_author_onclick = function() {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            pushNotification("Запрос о добавлении автора успешно отправлен", "72D38C");
        } else if (this.readyState != 4) {
            pushNotification("Запрос о добавлении автора обрабатывается", "88ACFF");
        } else {
            pushNotification("Произошла ошибка. Ответ сервера:" + this.status, "D17373");
        }
    }

    xhr.open("POST","/api/newAuthor");
    xhr.setRequestHeader('Content-Type', 'application/json');

    xhr.send(JSON.stringify(
        {
            'name': document.querySelector('#authorForm > div > input.formName').value,
            'birthday': document.querySelector('#authorForm > div > input.formDate').value,
            'zodiac_id': document.querySelector('#authorForm > div > input.formZodiac').value
        }
    ));
};

const new_ordermaker_onclick = function(){
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            pushNotification("Запрос о добавлении заказчика успешно отправлен", "72D38C");
        } else if (this.readyState != 4) {
            pushNotification("Запрос о добавлении закачика обрабатывается", "88ACFF");
        } else {
            pushNotification("Произошла ошибка. Ответ сервера:" + this.status, "D17373");
        }
    }

    xhr.open("POST","/api/newOrdermaker");
    xhr.setRequestHeader('Content-Type', 'application/json');

    let is_organization = document.querySelector('#ordermakerForm > div > input.formIsOrganization').value;

    xhr.send(JSON.stringify(
        {
            'is_organization': (is_organization=='on')? true : false,
            'contact_name': document.querySelector('#ordermakerForm > div > input.formContactName').value,
            'address': document.querySelector('#ordermakerForm > div > input.formAddress').value,
            'phone': document.querySelector('#ordermakerForm > div > input.formPhone').value,
            'title': document.querySelector('#ordermakerForm > div > input.formTitle').value
        }
    ));
}

let xhr_have_rights = new XMLHttpRequest();
xhr_have_rights.onreadystatechange = async function() {
    if (this.readyState == 4 && this.status == 200) {
        const json = Number(JSON.parse(xhr_have_rights.responseText));
        if (json>1) {
            addMenuButton.style.color = "white";
            addMenuButton.disabled = false;
        } else {
            addButton.disabled = true;
            editButton.disabled = true;
            viewButton.disabled = true;
        }
        rightsResult = json;
    }
}
xhr_have_rights.open("GET", "/api/haveRights", true);
xhr_have_rights.send();



// block on while not xhr have rights is responsed

const haveRights = lockOnRights().then((res)=>res).catch((err)=>console.error(err));

listMenuButton.onclick = () => {
    form.style.display = "none";
    ordersList.style.display="flow-root";
    undermenu.style.display="none";
    journal.style.display = 'none';
}

handleMenuButton.onclick = () => {
    form.style.display="none";
    ordersList.style.display="none";
    undermenu.style.display="flow-root";
}

async function init_forms() {
    let x = await haveRights;
    if (x > 1) {
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

        function newButton(fn_onclick) {
            // template for further buttons
            let newButton = document.createElement("button");
            newButton.classList.add("menuButton");
            newButton.classList.add("formButton");
            newButton.innerText="Отправить";
            newButton.onclick = fn_onclick;
            return newButton;
        }

        orderForm.innerHTML += `<div class="formHeader"> Новый заказ </div>`;
        orderForm.innerHTML += `<div> Название <input class="formInput formName"></input></div>`;
        orderForm.innerHTML += `<div> Год <input class="formInput formYear"></input></div>`;
        orderForm.innerHTML += `<div> Цена <input class="formInput formPrice"></input></div>`;

        typoForm.innerHTML += `<div class="formHeader"> Новая типография </div>`;
        typoForm.innerHTML += `<div> Название <input class="formInput formName" id="formName"></input></div>`;
        typoForm.innerHTML += `<div> Адрес    <input class="formInput formAddress" id="formAddress"></input></div>`;
        typoForm.innerHTML += `<div> Телефон  <input class="formInput formPhone" id="formPhone" ></input></div>`;

        authorForm.innerHTML += `<div class="formHeader"> Новый автор </div>`;
        authorForm.innerHTML += `<div> ФИО <input class="formInput formName"></input></div>`;
        authorForm.innerHTML += `<div> Дата рождения <input class="formInput formDate" type="date"></input></div>`;
        authorForm.innerHTML += `<div> Зодиаки:
        <div class="zodiac">1 - Неизвестен</div>
        <div class="zodiac">2 - Овен 	Март 21 - Апрель 20 	♈</div>
        <div class="zodiac">3 - Телец 	Апрель 21 - Май 20 	♉</div>
        <div class="zodiac">4 - Близнецы 	Май 21 - Июнь 21 	♊</div>
        <div class="zodiac">5 - Рак 	Июнь 22 - Июль 22 	♋</div>
        <div class="zodiac">6 - Лев 	Июль 23 - Август 22 	♌</div>
        <div class="zodiac">7 - Дева 	Август 23 - Сентябрь 22 	♍</div>
        <div class="zodiac">8 - Весы 	Сентябрь 23 - Октябрь 22 	♎</div>
        <div class="zodiac">9 - Скорпион 	Октябрь 23 - Ноябрь 22 	♏</div>
        <div class="zodiac">10 - Стрелец 	Ноябрь 23 - Декабрь 21 	♐</div>
        <div class="zodiac">11 - Козерог 	Декабрь 22 - Январь 20 	♑</div>
        <div class="zodiac">12 - Водолей 	Январь 21 - Февраль 18 	♒</div>
        <div class="zodiac">13 - Рыбы 	Февраль 19 - Март 20 	♓ </div></div>`;
        authorForm.innerHTML += `<div> Номер зодиака <input class="formInput formZodiac" type="number"></input></div>`;

        ordermakerForm.innerHTML += `<div class="formHeader"> Новый автор </div>`;
        ordermakerForm.innerHTML += `<div> Организация? <input type="checkbox" class="formInput formIsOrganization"></input></div>`;
        ordermakerForm.innerHTML += `<div> Имя контактного лица: <input class="formInput formContactName"></input></div>`;
        ordermakerForm.innerHTML += `<div> Адрес <input class="formInput formAddress"></input></div>`;
        ordermakerForm.innerHTML += `<div> Телефон <input class="formInput formPhone"></input></div>`;
        ordermakerForm.innerHTML += `<div> Условное название <input class="formInput formTitle"></input></div>`;


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


        const newTypographyButton = newButton(new_typography_onclick);
        const newOrderButton = newButton(new_order_onclick);
        const newAuthorButton = newButton(new_author_onclick);
        const newOrdermakerButton = newButton(new_ordermaker_onclick);

        typoForm.appendChild(newTypographyButton);
        orderForm.appendChild(newOrderButton);
        authorForm.appendChild(newAuthorButton);
        ordermakerForm.appendChild(newOrdermakerButton);
    }
}
let _promise = init_forms();

addMenuButton.onclick = async function() {
    let x = await haveRights;

    if(x > 1) {
        if(form.childNodes.length > 0) {
            form.removeChild(form.childNodes[0]);
        }
        form.style.display = "flow-root";
        form.appendChild(orderForm);
        ordersList.style.display="none";
        undermenu.style.display="none";
        journal.style.display = 'none';
    }
}




let newTypographyButton = document.createElement('button');
newTypographyButton.id = 'newTypography';
newTypographyButton.innerText = 'Типография';
newTypographyButton.classList.add('nav-add');
newTypographyButton.onclick = function() {
    if(form.childNodes.length > 0) {
        form.removeChild(form.childNodes[0]);
    }
    form.style.display = "flow-root";
    form.appendChild(typoForm);
}
let newAuthorButton = document.createElement('button');
newAuthorButton.id = 'newAuthor';
newAuthorButton.innerText = 'Автор';
newAuthorButton.classList.add('nav-add');
newAuthorButton.onclick = function() {
    if(form.childNodes.length > 0) {
        form.removeChild(form.childNodes[0]);
    }
    form.style.display = "flow-root";
    form.appendChild(authorForm);
}
let newOrdermakerButton = document.createElement('button');
newOrdermakerButton.id = 'newOrdermaker';
newOrdermakerButton.innerText = 'Заказчик';
newOrdermakerButton.classList.add('nav-add');
newOrdermakerButton.onclick = function() {
    if(form.childNodes.length > 0) {
        form.removeChild(form.childNodes[0]);
    }
    form.style.display = "flow-root";
    form.appendChild(ordermakerForm);
};

newTypographyButton.style.display="none";
newAuthorButton.style.display="none";
newOrdermakerButton.style.display="none";

undermenu.appendChild(newTypographyButton);
undermenu.appendChild(newAuthorButton);
undermenu.appendChild(newOrdermakerButton);

addButton.onclick = async function() {
    addButton.innerText = "Добавить >";

    let x = await haveRights;

    if(x > 1){
        newTypographyButton.style.display="flow-root";
        newAuthorButton.style.display="flow-root";
        newOrdermakerButton.style.display="flow-root";
    } else {
        pushNotification("У вас нет прав на просмотр этой страницы", "D17373");
    }
    viewButton.innerText = "Просмотр";
    editButton.innerText = "Изменить";
    journal.style.display = 'none';
    logoutButton.style.display="none";
    whoami.style.display="none";
}

viewButton.onclick = async function() {
    logoutButton.style.display="none";
    whoami.style.display="none";
    form.style.display = "none";
    addButton.innerText = "Добавить";
    newTypographyButton.style.display="none";
    newAuthorButton.style.display="none";
    newOrdermakerButton.style.display="none";
    viewButton.innerText = "Просмотр >";
    editButton.innerText = "Изменить";

    if (journal.childNodes.length > 0) {
        journal.style.display = "flow-root";
        return;
    }

    let x = await haveRights;

    if (x > 2) {
        let xhr = new XMLHttpRequest();
        xhr.onreadystatechange = function() {
            if (xhr.readyState==4 && xhr.status==200) {
                console.log(this.responseText);
                collection = JSON.parse(this.responseText);
                let new_li = document.createElement('ul');
                new_li.className = 'activeOrders';
                for(let i=0; i<collection.length; i++) {
                    let new_ul = document.createElement('li');
                    new_ul.className = 'activeOrder';
                    new_ul.innerText += 'ID: ' + collection[i]['id'] + ': ' + collection[i]['action'] + ', дата: ' + collection[i]['date'];
                    new_li.appendChild(new_ul);
                }
                journal.appendChild(new_li);
                journal.style.display = "flow-root";
            } else if (xhr.readyState!=4) {
                pushNotification("Загрузка журнала", "88ACFF");
            }
        }
        xhr.open('GET','/api/getActions',true);
        xhr.send();
    } else {
        pushNotification("У вас нет прав на просмотр этой страницы", "D17373");
    }
}

editButton.onclick = function() {
    form.style.display = "none";
    addButton.innerText = "Добавить";
    newTypographyButton.style.display="none";
    newAuthorButton.style.display="none";
    newOrdermakerButton.style.display="none";
    viewButton.innerText = "Просмотр";
    editButton.innerText = "Изменить >";
    journal.style.display = 'none';
    logoutButton.style.display="none";
    whoami.style.display="none";
}

let whoami = document.createElement("div");
whoami.className = "nav-add";
whoami.style.display="none";
undermenu.appendChild(whoami);
let logoutButton = document.createElement('button');
logoutButton.className = "nav-add";
logoutButton.innerText = "Выход";
logoutButton.onclick = function() {
    document.cookie = "hash=null"+";SameSite=Lax;Secure";
    document.location = "/login";
};
logoutButton.style.display="none";
undermenu.appendChild(logoutButton);

accountButton.onclick = async function() {
    form.style.display = "none";
    addButton.innerText = "Добавить";
    newTypographyButton.style.display="none";
    newAuthorButton.style.display="none";
    newOrdermakerButton.style.display="none";
    viewButton.innerText = "Просмотр";
    editButton.innerText = "Изменить";
    journal.style.display = 'none';
    logoutButton.style.display="flow-root";
    let x = await haveRights;
    whoami.innerHTML = "<p>Мой уровень прав: " + x+`</p>
    <p>Куки: <span class="covered">`+document.cookie+`</span></p>`;
    whoami.style.display="flow-root";
}
