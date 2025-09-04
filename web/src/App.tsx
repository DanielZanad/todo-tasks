import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Login } from "./pages/login";
import { SignUp } from "./pages/sign-up";
import { ProtectedRoute } from "./pages/protected-route";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Home } from "./pages/home";

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route
            index
            element={
              <ProtectedRoute>
                <Home />
              </ProtectedRoute>
            }
          />
          <Route path={"/login"} element={<Login />} />
          <Route path={"/sign-up"} element={<SignUp />} />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}

export default App;
