use cursive::views::{Dialog, EditView, TextView, LinearLayout, Button,
                     SelectView};
use cursive::traits::*;
use cursive::Cursive;

use crate::{client_storytel_api, mpv};

fn show_player(siv: &mut Cursive, book_id: &u64) {
    let client_data =
        siv.user_data::<client_storytel_api::ClientData>().unwrap();
    let url_ask_stream = client_storytel_api::get_stream_url(client_data,
                                                             book_id);

    let resp = client_data.request_client.get(&url_ask_stream)
        .send();

    let location = resp.as_ref().unwrap().url().to_owned().into_string();

    if let Some(sender) = client_data.mpv_thread.as_ref() {
        sender.send(true).unwrap();
    }

    client_data.mpv_thread = Some(mpv::simple_example(location));
}

fn show_bookshelf(siv: &mut Cursive) {
    let bookshelf = client_storytel_api::get_bookshelf(
        siv.user_data::<client_storytel_api::ClientData>().unwrap());
    siv.pop_layer();
    let mut book_select: Vec<(String, u64)> = Vec::new();
    for book_entry in bookshelf.books.iter() {
        match &book_entry.abook {
            Some(abook) => book_select.push((book_entry.book.name.clone(),
                                             abook.id)),
            None    => continue,
        }
        println!("{}", book_entry.book.name);
    }
    let select = SelectView::new()
        .with_all(book_select.into_iter())
        .on_submit(show_player);
    siv.add_layer(
        Dialog::around(select.scrollable())
            .title("Select a book to listen"),
    );
}

fn show_check_login(siv: &mut Cursive, email: &str, pass: &str) {
    if email.is_empty() {
        siv.add_layer(Dialog::info("Please enter a email!"));
    } else if pass.is_empty() {
        siv.add_layer(Dialog::info("Please enter a password!"));
    } else {
        client_storytel_api::login(
            siv.user_data::<client_storytel_api::ClientData>().unwrap(),
            email, pass);
        siv.pop_layer();
        siv.add_layer(Dialog::around(LinearLayout::vertical()
                .child(Button::new("Bookshelf", show_bookshelf))
                .child(Button::new("Exit", show_login)))
            .title("Menu"));
    }
}

pub fn show_login(siv: &mut Cursive) {
    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new("Email"))
            .child(
                EditView::new()
                    .with_name("email")
                    .fixed_width(20)
                )
            .child(TextView::new("Password"))
            .child(
                EditView::new()
                    .secret()
                    .with_name("pass")
                    .fixed_width(20)
                ))
        .button("Ok", |s| {
                let email = s
                    .call_on_name("email", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                let pass = s
                    .call_on_name("pass", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                show_check_login(s, &email, &pass);
        })
        .title("Login"));
}

