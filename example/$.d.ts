declare namespace ${
    function print(...args: any[]);
    function println(...args: any[]);
    // function assert(assertion: boolean, reason: String): null;
    // function err(...args: Any[]): null;
    // function errln(...args: Any[]): null;
    // function format(...args: Any[]): String;
    // function formatln(...args: Any[]): String;
    // function callback_test(): Promise<String>;
    function file_read(filename: String): Promise<String>
}
export default $