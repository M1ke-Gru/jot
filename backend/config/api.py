# config/api.py
from ninja import NinjaAPI
from apps.tasks.api import router as tasks_router
from apps.users.api import router as users_router

api = NinjaAPI(title="My app API")
api.add_router("/tasks", tasks_router)
api.add_router("/users", users_router)
