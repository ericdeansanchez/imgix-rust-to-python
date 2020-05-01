# -*- coding: utf-8 -*-
import os

import pytest
import imgix 

def test_url():
    u = imgix.Url("test.imgix.net").path("image.png")
    assert u.get_path() == "image.png"