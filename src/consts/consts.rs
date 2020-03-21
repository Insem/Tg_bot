pub const SERVER_IP: [u8; 4] = [127, 0, 0, 1];
pub fn CMD_LIST() ->std::vec::Vec<&'static str> {
    vec![
        "start"
        ]
}
pub const SERVER_PORT: u16 = 8080;
pub const TOKEN: &str = "bot1063777732:AAEnAMLJZmiOg-q7D5OW8AA1DpTFzQTX4rw";

/*                "{\"update_id\":473131509,
                \n\"message\":{
                    \"message_id\":25,
                    \"from\":{
                        \"id\":180249842,
                        \"is_bot\":false,
                        \"first_name\":\"\\u0414\\u0435\\u043a\\u0441\\u0442\\u0435\\u0440 (\\u0443 \\u043c\\u0435\\u043d\\u044f \\u0432\\u0430\\u0449\\u0435\\u0442 \\u0438\\u043c\\u044f \\u0435\\u0441\\u0442\\u044c)\",
                        \"last_name\":\"\\u041b\\u0435\\u043a\\u0442\\u043e\\u0440\\u043e\\u0432\\u0438\\u0447\",
                        \"username\":\"insem\",
                        \"language_code\":\"ru\"
                    },
                    \"chat\":{
                        \"id\":180249842,
                        \"first_name\":\"\\u0414\\u0435\\u043a\\u0441\\u0442\\u0435\\u0440 (\\u0443 \\u043c\\u0435\\u043d\\u044f \\u0432\\u0430\\u0449\\u0435\\u0442 \\u0438\\u043c\\u044f \\u0435\\u0441\\u0442\\u044c)\",
                        \"last_name\":\"\\u041b\\u0435\\u043a\\u0442\\u043e\\u0440\\u043e\\u0432\\u0438\\u0447\",
                        \"username\":\"insem\",\"type\":\"private\"
                    },
                        \"date\":1581183917,
                        \"text\":\"/start\",
                        \"entities\":[{\"offset\":0,\"length\":6,\"type\":\"bot_command\"}]}}"*/