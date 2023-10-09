import { Route, Router, Routes } from "@solidjs/router";
import Main from "./views/Main";
import Toolbar from "./layout/Toolbar";
import Proclist from "./views/Proclist";

function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" component={Toolbar}>
                    <Route path="" component={Main} />
                    <Route path="/proclist" component={Proclist} />
                </Route>
            </Routes>


        </Router>
    );
}

export default App;
