Overall goal is to have a full mangadex api written in rust for whatever purpose lmao

## Goals
* ~~get manga data from site~~
* add file writing
* ~~replace `reqwest::Error` with `mangadex::Error`~~
* ~~add 404 errors and related~~
* add user account and searching
* add async
* add caching (is this too far out of my jurisdiction as an api?)
* add posting

## Links

```
https://mangadex.org/api/manga/<id>
	https://mangadex.org/api/chapter/<id>
		https://s4.mangadex.org/data/<chapterhash>/<page>
https://mangadex.org/images/flags/<countrycode>.png
```

### Za Hando
those warnings \n 
with these
```
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
```
god sned