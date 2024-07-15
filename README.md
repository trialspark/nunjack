nunjack
=======

nunjack is a generic health check executor. It can perform an assortment of operations meant to test the health of an application or service and return a standard exit code to indicate the status of the test subject.

Tests
-----

	nunjack exec <script> [args]
	
	nunjack pid <pid>
	
	nunjack pid-file <file>
	
	nunjack socket <path>

Return
------

	0	Success
	10	IO Error
	65	User Error
	69	Unavailable
	77	Permission Rrror
	255	Unknown

