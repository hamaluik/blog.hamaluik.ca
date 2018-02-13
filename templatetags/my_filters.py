# -*- coding:utf-8 -*-
from __future__ import unicode_literals
from statik.templatetags import register
from slugify import slugify
import pytz

@register.filter(name='slugify')
def filter_slugify(s):
    return slugify(s)

@register.filter(name='atomdate')
def filter_atomdate(d):
    local = pytz.timezone("America/Edmonton")
    ld = local.localize(d, is_dst=None)
    #ud = ld.astimezone(pytz.utc)
    return ld.isoformat('T')
