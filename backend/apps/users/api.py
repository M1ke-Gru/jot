# apps/users/api.py
from django.contrib.auth import authenticate, login, logout, get_user_model
from django.contrib.auth.password_validation import validate_password
from django.core.exceptions import ValidationError
from ninja import Router, Schema
from ninja.security import SessionAuth
from pydantic import UUID4, EmailStr

router = Router(tags=["auth"])
auth = SessionAuth()


class LoginIn(Schema):
    email: EmailStr
    password: str


class SignUp(LoginIn):
    username: str


class UserOut(Schema):
    id: UUID4
    email: EmailStr


#TODO: rate limiting [security]
#TODO: csrf [security]
@router.post(
    "/signup",
    auth=None,
    response={201: UserOut, 400: dict, 409: dict},
)
def signup(request, data: SignUp):
    User = get_user_model()

    if User.objects.filter(username=data.username).exists():
        return 409, {"detail": "Username already taken"}

    try:
        validate_password(data.password)
    except ValidationError as exc:
        return 400, {"detail": exc.messages}

    user = User.objects.create_user(
        username=data.username,
        email=data.email,
        password=data.password,
    )
    return 201, user


@router.post("/login", auth=None)
def login_api(request, data: LoginIn):
    user = authenticate(
        request,
        email=data.email,
        password=data.password,
    )

    if user is None:
        return 401, {"detail": "Invalid username or password"}

    login(request, user)  # Django sends a secure session cookie
    return {"id": user.id, "username": user.username}


@router.get("/me", auth=auth)
def me(request):
    user = request.auth
    return {"id": user.id, "username": user.username}


@router.post("/logout", auth=auth)
def logout_api(request):
    logout(request)
    return {"ok": True}
