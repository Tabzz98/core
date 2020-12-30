/*
 *	MetaCall Library by Parra Studios
 *	A library for providing a foreign function interface calls.
 *
 *	Copyright (C) 2016 - 2020 Vicente Eduardo Ferrer Garcia <vic798@gmail.com>
 *
 *	Licensed under the Apache License, Version 2.0 (the "License");
 *	you may not use this file except in compliance with the License.
 *	You may obtain a copy of the License at
 *
 *		http://www.apache.org/licenses/LICENSE-2.0
 *
 *	Unless required by applicable law or agreed to in writing, software
 *	distributed under the License is distributed on an "AS IS" BASIS,
 *	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *	See the License for the specific language governing permissions and
 *	limitations under the License.
 *
 */

#include <gmock/gmock.h>

#include <metacall/metacall.h>
#include <metacall/metacall_value.h>
#include <metacall/metacall_loaders.h>

class metacall_node_python_port_mock_test : public testing::Test
{
public:
};

TEST_F(metacall_node_python_port_mock_test, DefaultConstructor)
{
	metacall_print_info();

	metacall_log_null();

	ASSERT_EQ((int) 0, (int) metacall_initialize());

	/* NodeJS & Python & Mock */
	#if defined(OPTION_BUILD_LOADERS_NODE) && defined(OPTION_BUILD_LOADERS_PY) && defined(OPTION_BUILD_LOADERS_MOCK)
	{
		static const char buffer[] =
			/* NodeJS */
			"const { metacall, metacall_load_from_memory } = require('" METACALL_NODE_PORT_PATH "');\n"
			"metacall_load_from_memory('py', `"
				/* Python */
				"import sys\n"
				"sys.path.insert(0, '" METACALL_PYTHON_PORT_PATH "')\n"
				"import metacall\n"
				/* Mock */
				"from asd.mock import two_doubles\n"
				"def py_func(js_func):\n"
				"	return js_func(two_doubles)\n"
			"`);\n"
			"const result = metacall('py_func', (mock_func) => mock_func(3, 4));\n"
			"console.log('Result:', result);\n"
			"if (result !== 3.1416) process.exit(1);\n";

		ASSERT_EQ((int) 0, (int) metacall_load_from_memory("node", buffer, sizeof(buffer), NULL));
	}
	#endif /* OPTION_BUILD_LOADERS_NODE && OPTION_BUILD_LOADERS_PY && OPTION_BUILD_LOADERS_MOCK */

	EXPECT_EQ((int) 0, (int) metacall_destroy());
}