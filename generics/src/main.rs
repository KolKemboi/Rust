#[allow(unused)]

fn main()
{
    let data = Data::new("info", "endpoint");
    data.ret_endpoint();
    data.ret_info();
}

trait InfoShow
{
    fn showinfo(&self) ->String;
}
struct Data<T, U>
{
    info: T,
    endpoint: U,
}

impl <T: std::fmt::Display, U: std::fmt::Display> Data<T, U>
{
    fn new(info: T, endpoint: U) ->Self
    {
        Data { info: info, endpoint: endpoint }
    }    
    fn ret_info(&self)
    {
        println!("{}", self.info);
    }
    fn ret_endpoint(&self)
    {
        println!("{}", self.endpoint);
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> InfoShow for Data<T, U>
{
    fn showinfo(&self) -> String {
        format!("Info: {}, Endpoint: {}", self.info, self.endpoint)
    }
}