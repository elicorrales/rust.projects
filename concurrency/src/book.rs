pub struct BookCopy {
    pub id: u32,
    pub author: String
}

pub struct Book {
    pub id: u32,
    pub author: &'static str
}

pub static BOOKS:[Book;40] = [

    Book { id: 1, author: "Name1 Looooonnnng LastName" },
    Book { id: 2, author: "Name2 Looooonnnng LastName" },
    Book { id: 3, author: "Name3 Looooonnnng LastName" },
    Book { id: 4, author: "Name4 Looooonnnng LastName" },
    Book { id: 5, author: "Name5 Looooonnnng LastName" },
    Book { id: 6, author: "Name6 Looooonnnng LastName" },
    Book { id: 7, author: "Name7 Looooonnnng LastName" },
    Book { id: 8, author: "Name8 Looooonnnng LastName" },
    Book { id: 9, author: "Name9 Looooonnnng LastName" },
    Book { id: 10, author: "Name10 Looooonnnng LastName" },
    Book { id: 11, author: "Name11 Looooonnnng LastName" },
    Book { id: 12, author: "Name12 Looooonnnng LastName" },
    Book { id: 13, author: "Name13 Looooonnnng LastName" },
    Book { id: 14, author: "Name14 Looooonnnng LastName" },
    Book { id: 15, author: "Name15 Looooonnnng LastName" },
    Book { id: 16, author: "Name16 Looooonnnng LastName" },
    Book { id: 17, author: "Name17 Looooonnnng LastName" },
    Book { id: 18, author: "Name18 Looooonnnng LastName" },
    Book { id: 19, author: "Name19 Looooonnnng LastName" },
    Book { id: 20, author: "Name20 Looooonnnng LastName" },

    Book { id: 21, author: "Name21 Looooonnnng LastName" },
    Book { id: 22, author: "Name22 Looooonnnng LastName" },
    Book { id: 23, author: "Name23 Looooonnnng LastName" },
    Book { id: 24, author: "Name24 Looooonnnng LastName" },
    Book { id: 25, author: "Name25 Looooonnnng LastName" },
    Book { id: 26, author: "Name26 Looooonnnng LastName" },
    Book { id: 27, author: "Name27 Looooonnnng LastName" },
    Book { id: 28, author: "Name28 Looooonnnng LastName" },
    Book { id: 29, author: "Name29 Looooonnnng LastName" },
    Book { id: 30, author: "Name30 Looooonnnng LastName" },
    Book { id: 31, author: "Name31 Looooonnnng LastName" },
    Book { id: 32, author: "Name32 Looooonnnng LastName" },
    Book { id: 33, author: "Name33 Looooonnnng LastName" },
    Book { id: 34, author: "Name34 Looooonnnng LastName" },
    Book { id: 35, author: "Name35 Looooonnnng LastName" },
    Book { id: 36, author: "Name36 Looooonnnng LastName" },
    Book { id: 37, author: "Name37 Looooonnnng LastName" },
    Book { id: 38, author: "Name38 Looooonnnng LastName" },
    Book { id: 39, author: "Name39 Looooonnnng LastName" },
    Book { id: 40, author: "Name40 Looooonnnng LastName" },

];

