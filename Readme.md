# Image pool - сервер для загрузки изображений, основан на actix-web.

# Запуск (docker):

Из директории с проектом:
```` 
docker build -t imgpool .
docker run --rm --name imgpool --network="host" -v "$PWD/uploads":/uploads imgpool
````
Сервер станет доступен по адрессу http://127.0.0.1:8088/images (изображения будут находится в папке uploads)

# Инфо

Поддерживаются форматы png и jpg. Graсeful shutdown - 30 сек после kill -15 (SIGTERM).
Сервер обрабатывает Put запросы с content-type application/json или multipart/form-data.<br>
Json:
````
{
	"file": [{base64 decoded files here}],
	"uri": [{uri's here}]
}
````

Form:
````
file[] = ...
file[] = ...
uri[] = ...
uri[] = ...
````

Примеры curl запросов в директории examples.

Сервер возвращает json массив с объектами содержащими информацию по каждому входному файлу.
Пр:
````
[
  {
    "ok": true,
    "success_info": {
      "original_path": "./uploads/f347ed2c-a7bc-4a45-b92c-25f9fcdbff07.png",
      "resize_path": "./uploads/40dff0ba-3b8c-4d60-a584-ba663b343c08.png"
    },
    "fail_info": null
  },
  {
    "ok": true,
    "success_info": {
      "original_path": "./uploads/31051db8-3d5c-4862-9a48-198ca02c278f.png",
      "resize_path": "./uploads/cbe357c1-14d1-4fcc-96e8-4c2866ea7a8d.png"
    },
    "fail_info": null
  }
]
````

В случае сохранение какого либо файла не удалось, соотв. объект будет иметь вид
<br>Пр: (не смогли достать изображение по url)
````
{
    "ok": false,
    "success_info": null,
    "fail_info": {
      "reason": "Unsupported resource http://avatars.mds.yandex.net/get-pdb/251121/009892a0-e2b8-4cc8-b718-d9f61a2ea4fc/s1200312 !"
    }
  }
````
# Тесты

Сервер:
```` 
docker exec -it imgpool bash -c "cd \img-pool && cargo test"
```` 

Opencv крейт:
```` 
docker exec -it imgpool bash -c "cd \img-pool/opencv && cargo test"
```` 

