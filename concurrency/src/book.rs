pub struct BookCopy {
    pub id: u32,
    pub author:String
}

pub struct Book {
    pub id: u32,
    pub author: &'static str
}



pub static BOOKS:[Book;20] = [
    Book { id: 1, author: "Name1" },
    Book { id: 2, author: "Name2" },
    Book { id: 3, author: "Name3" },
    Book { id: 4, author: "Name4" },
    Book { id: 5, author: "Name5" },
    Book { id: 6, author: "Name6" },
    Book { id: 7, author: "Name7" },
    Book { id: 8, author: "Name8" },
    Book { id: 9, author: "Name9" },
    Book { id: 10, author: "Name10" },
    Book { id: 11, author: "Name11" },
    Book { id: 12, author: "Name12" },
    Book { id: 13, author: "Name13" },
    Book { id: 14, author: "Name14" },
    Book { id: 15, author: "Name15" },
    Book { id: 16, author: "Name16" },
    Book { id: 17, author: "Name17" },
    Book { id: 18, author: "Name18" },
    Book { id: 19, author: "Name19" },
    Book { id: 20, author: "Name20" },
];


