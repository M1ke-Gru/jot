from django.test import TestCase
from apps.users.models import User


class TasksTest(TestCase):
    def setUp(self) -> None:
        self.user = User.objects.create_user(
            username="alex",
            email="alex@example.com",
            password="test-password-123",
        )
        self.client.force_login(self.user)

    def test_success_task_list(self) -> None:
        response = self.client.get("/api/tasks/")
        self.assertEqual(response.status_code, 200, response.content)

    def test_unauthenticated_user_cannot_list_tasks(self) -> None:
        self.client.logout()
        response = self.client.get("/api/tasks/")
        self.assertEqual(response.status_code, 401, response.content)
