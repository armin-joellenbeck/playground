fn main() {
    let links = Links {
        path: String::from("/"),
        items: vec![
            Link {
                href: String::from("mailto:armin@joellenbeck.net"),
                text: String::from("armin@joellenbeck.net"),
            },
            Link {
                href: String::from("https://github.com/armin-joellenbeck"),
                text: String::from("GitHub Profile"),
            },
            Link {
                href: String::from("/papers"),
                text: String::from("Mathematical Papers"),
            },
        ],
    };

    let resources = vec![
        links,
    ];

    for resource in resources {
        println!("{}index.html", resource.path);
        println!("{}", resource.html());
    }
}

struct Link {
    href: String,
    text: String,
}

impl Link {
    fn html(&self) -> String {
        format!(
            "<a href=\"{}\">{}</a>",
            self.href,
            self.text
        )
    }
}

struct Links {
    path: String,
    items: Vec<Link>,
}

impl Links {
    fn html(&self) -> String {
        format!(
            "<ul>\n{}</ul>",
            self.items.iter().fold(String::new(),
                move |acc, item| format!("{}  <li>\n    {}\n  </li>\n", acc, item.html())
            )
        )
    }
}
