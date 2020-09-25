declare namespace std {
    declare function print(...args: any[]): void;
    declare function println(...args: any[]): void;
    function assert(assertion: boolean, reason: String): null;
    // function err(...args: Any[]): null;
    // function errln(...args: Any[]): null;
    // function format(...args: Any[]): String;
    // function formatln(...args: Any[]): String;
    // function callback_test(): Promise<String>;
    let file = {
        open: function open(filename: String): Promise<String>;

    }
  
}
