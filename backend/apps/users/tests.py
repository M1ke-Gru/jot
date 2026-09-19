from django.test import TestCase
from django.contrib.auth import authenticate
from django.contrib.auth import get_user_model


class EmailAuthenticationTests(TestCase):
    def test_authenticates_with_email_and_password(self):
        user = get_user_model().objects.create_user(
            username="mike",
            email="mike@example.com",
            password="correct-horse-battery-staple",
        )

        authenticated_user = authenticate(
            email="MIKE@example.com",
            password="correct-horse-battery-staple",
        )

        self.assertEqual(authenticated_user, user)
