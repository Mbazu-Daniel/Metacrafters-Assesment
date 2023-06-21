import { useGlobalState } from "./store";






const  background = 'https://images.unsplash.com/photo-1685544830644-f2966cdb4aad?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1470&q=80'
const Hero = () => {
  const [connectedAccount] = useGlobalState('connectedAccount')

  return(
    <div
    className=" p-60 md:px-40 "
    style={{ background: `url('${background}') fixed no-repeat top/cover `}}
    >
      <div className="flex  items-center justify-between text-white py-5">
        <div>
          <h1 className="text-3xl font-bold"> Streamline Token Distribution:  </h1>
          <h2 className="text-3xl font-bold mt-1">Register Organizations and Tokens quickly </h2>
          <h3 className="text-xl font-semibold mt-6"> Tailor Vesting Periods for Success: Customize Rewards for Stakeholders</h3>
          <p className="mt-2" >Granting Rewards After the Vesting Period</p>
        </div>
        <div  className="hidden lg:flex items-center space-x-3 font-semibold opacity-50 ">
          
          
          </div>
          </div>
          {connectedAccount ? (
            <div className="flex justify-between items-center ">
            
            <div className=" flex justify-start items-start pb-5">
              <a
              href={'/Admin'}
              className="bg-violet-600 hover:bg-violet-700 text-white rounded-xl cursor-pointer uppercase shadow-md
              shadow-gray-600 font-bold p-3"
              >Organization Login
              </a>
            </div>
            <div className=" flex pb-5 ">
              <a
              href={'/User'}
              className="bg-violet-600 hover:bg-violet-700 text-white rounded-xl cursor-pointer uppercase shadow-md
              shadow-gray-600 font-bold p-3"
              >User Login
              </a>
            </div>
            </div>
              ) : (
                <div className="flex justify-center items-center text-yellow-200 text-4xl font-semibold mt-6">
                  Connect Wallet, Register or/and Claim Token
                </div>
              )}
          
          
          
    </div>
  ) 
};

export default Hero;
