UPDATE user
SET email     = coalesce(?1, email),
    password  = coalesce(?2, password),
    firstname = coalesce(?3, firstname),
    lastname  = coalesce(?4, lastname),
    nickname  = coalesce(?5, nickname),
    image     = IIF(?6 IS NULL, image, IIF(?6 IS '', NULL, ?6)),
    about     = coalesce(?7, about),
    birthday  = coalesce(?8, birthday),
    private   = coalesce(?9, private)

WHERE userID = ?;
