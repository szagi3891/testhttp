/*
token        -> (connection, Option<timeout>)


ReadingRequest    - wtedy startujemy z timeoutem    oczekiwanie na dane od użytkownika
SendingResponse - -||-                            wysyłanie danych użytkownikowi


parsowanie, oczekiwanie na serwer - te timeouty już backend musi sobie zaimplementować


miohttp - nowa instancja ma startować z tymi parametrami czasowymi


        match self.hash.remove(&token) {

            Some(connection) => {

                let new_connection = connection.send_data_to_user(event_loop, token.clone(), response);

                self.hash.insert(token.clone(), new_connection);
            }

            None => {
                println!("socket_ready: no socket by token: {:?}", &token);        -- ten komunikat będzie zaszyty w funkcji transform
            }
        }


        match self.hash.transform(&token, |connection|{

            let new_connection = connection.send_data_to_user(event_loop, token.clone(), response);

            new_connection
        });

            ta funkcja będzie posiadała kontekst, więc będzie wiedziała czy zmienił się nowy stan
            na tej podstawie będzie mogła ustwić bądź wyzerować timer

*/

struct Hashmap {
    //HashMap<Token, Connection>

    map : HashMap<Token, (Connection)>,
}

