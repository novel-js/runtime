declare namespace std{
    function print(...args: Any[]);
    function println(...args: Any[]);
    // function assert(assertion: boolean, reason: String): null;
    // function err(...args: Any[]): null;
    // function errln(...args: Any[]): null;
    // function format(...args: Any[]): String;
    // function formatln(...args: Any[]): String;
    // function callback_test(): Promise<String>;
    function read(filename: String): Promise<String>
}