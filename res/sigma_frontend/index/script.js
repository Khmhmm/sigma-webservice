let hat = document.querySelector('.hat');
hat.onclick = function() {
    window.location = "/";
}

let usButton = document.getElementById('us')
let authorsButton = document.getElementById('authors')
let readButton = document.getElementById('read')
let cataloguesButton = document.getElementById('catalogue')
let merchButton = document.getElementById('merch')

let contentHeader = document.getElementById('content-header')
let contentText = document.getElementById('content-text')
let contentButton = document.getElementById('content-button')

let aboutUsHeader = 'О нас'
let aboutUsText = 'Мы, издательство “Сигма”, больше всего ценим слово. Ранящее до глубины души и вызывающее самые разные эмоции - оно сопровождает нас всю жизнь. Каждый из нас мыслит, знакомится и признается в любви, радуется новому дню и маленьким победам с помощью слов. Главная цель “Сигмы” - сохранить слова для современников и будущих поколений, а также помочь молодым авторам донести свои мысли и переживания до вас, наших читателей. Спасибо, что вы с нами, и добро пожаловать.'
let aboutUsButtonT = 'Отправиться в путешествие'

let authorsHeader = 'Кабинет'
let authorsText = 'С "Сигмой" сотрудничают как известные, так и начинающие авторы и организации - присоединяйтесь к нам уже сейчас, в пару кликов. Отслеживайте работу издательства в реальном времени уже сегодня - нет ничего проще.'
let authorsButtonT = 'Войти в кабинет'

// let catalogueHeader = 'Каталог'
// let catalogueText = 'Встречайте наши подборки на любой вкус от Кафки до Пелевина, от менеджмента до эзотерики, от ненависти до любви - один клик.'
// let catalogueButtonT = 'Пока недоступно'

let merchHeader = 'Мерч'
let merchText = 'Встречайте нашу коллекцию при поддержке слышкупитолсовку.рф'
let merchButtonT = 'Купить'

usButton.onclick = () => {
    contentHeader.innerText = aboutUsHeader
    contentText.innerText = aboutUsText
    contentButton.innerText = aboutUsButtonT
    contentButton.href = "/login"
}

authorsButton.onclick = () => {
    contentHeader.innerText = authorsHeader
    contentText.innerText = authorsText
    contentButton.innerText = authorsButtonT
    contentButton.href = "/login"
}

// cataloguesButton.onclick = () => {
//     contentHeader.innerText = catalogueHeader
//     contentText.innerText = catalogueText
//     contentButton.innerText = catalogueButtonT
//     contentButton.href = "/"
// }

merchButton.onclick = () => {
    contentHeader.innerText = merchHeader
    contentText.innerText = merchText
    contentButton.innerText = merchButtonT
    contentButton.href = "/"
}


console.log(usButton)
